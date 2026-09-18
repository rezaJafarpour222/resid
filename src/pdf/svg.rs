use std::collections::BTreeSet;

use lopdf::{Dictionary, Document, Object, ObjectId, Stream, content::Operation};

use crate::{error::AppError, pdf::writer::PdfWriter, units::Pt};

#[derive(Debug, Clone, Copy)]
pub struct SvgResource {
    pub id: ObjectId,

    pub width: Pt,

    pub height: Pt,
}

impl PdfWriter {
    pub fn embed_svg(&mut self, svg: &str) -> Result<SvgResource, AppError> {
        let mut options = svg2pdf::usvg::Options::default();

        options.fontdb_mut().load_system_fonts();

        let tree = svg2pdf::usvg::Tree::from_str(svg, &options)
            .map_err(|error| AppError::PdfWriter(format!("failed to parse SVG: {error}")))?;

        let svg_pdf = svg2pdf::to_pdf(
            &tree,
            svg2pdf::ConversionOptions::default(),
            svg2pdf::PageOptions::default(),
        )
        .map_err(|error| AppError::PdfWriter(format!("failed to convert SVG to PDF: {error}")))?;

        let mut svg_document = Document::load_mem(&svg_pdf).map_err(|error| {
            AppError::PdfWriter(format!("failed to read generated SVG PDF: {error}"))
        })?;

        let start_id = self
            .document
            .max_id
            .checked_add(1)
            .ok_or_else(|| AppError::PdfWriter("PDF object ID overflow".to_string()))?;

        svg_document.renumber_objects_with(start_id);

        let svg_page_id = *svg_document
            .get_pages()
            .get(&1)
            .ok_or_else(|| AppError::PdfWriter("generated SVG PDF has no page 1".to_string()))?;

        let svg_page = svg_document
            .get_dictionary(svg_page_id)
            .map_err(|error| AppError::PdfWriter(format!("failed to read SVG PDF page: {error}")))?
            .clone();

        let resources = svg_page
            .get(b"Resources")
            .map_err(|error| {
                AppError::PdfWriter(format!("SVG PDF page has no Resources: {error}"))
            })?
            .clone();

        let group = svg_page.get(b"Group").ok().cloned();

        let media_box = svg_page
            .get(b"MediaBox")
            .map_err(|error| AppError::PdfWriter(format!("SVG PDF page has no MediaBox: {error}")))?
            .as_array()
            .map_err(|error| {
                AppError::PdfWriter(format!("SVG PDF MediaBox is invalid: {error}"))
            })?;

        if media_box.len() != 4 {
            return Err(AppError::PdfWriter(
                "SVG PDF MediaBox must contain 4 values".to_string(),
            ));
        }

        let x_min = media_box[0]
            .as_float()
            .map_err(|error| AppError::PdfWriter(format!("invalid SVG MediaBox x_min: {error}")))?;

        let y_min = media_box[1]
            .as_float()
            .map_err(|error| AppError::PdfWriter(format!("invalid SVG MediaBox y_min: {error}")))?;

        let x_max = media_box[2]
            .as_float()
            .map_err(|error| AppError::PdfWriter(format!("invalid SVG MediaBox x_max: {error}")))?;

        let y_max = media_box[3]
            .as_float()
            .map_err(|error| AppError::PdfWriter(format!("invalid SVG MediaBox y_max: {error}")))?;

        let width = x_max - x_min;

        let height = y_max - y_min;

        if width <= 0.0 || height <= 0.0 {
            return Err(AppError::PdfWriter(format!(
                "invalid SVG size: {} x {}",
                width, height
            )));
        }

        let content = svg_document.get_page_content(svg_page_id);

        let mut referenced = BTreeSet::new();

        collect_referenced_objects(&svg_document, &resources, &mut referenced)?;

        if let Some(group) = &group {
            collect_referenced_objects(&svg_document, group, &mut referenced)?;
        }

        for object_id in referenced {
            let object = svg_document
                .objects
                .get(&object_id)
                .ok_or_else(|| {
                    AppError::PdfWriter(format!(
                        "SVG references missing PDF object {:?}",
                        object_id
                    ))
                })?
                .clone();

            self.document.objects.insert(object_id, object);
        }

        self.document.max_id = self.document.max_id.max(svg_document.max_id);

        let mut form = Dictionary::new();

        form.set("Type", "XObject");

        form.set("Subtype", "Form");

        form.set("FormType", 1);

        form.set(
            "BBox",
            vec![0.0.into(), 0.0.into(), width.into(), height.into()],
        );

        form.set("Resources", resources);

        if let Some(group) = group {
            form.set("Group", group);
        }

        let form_stream = Stream::new(form, content);

        let form_id = self.document.add_object(form_stream);

        Ok(SvgResource {
            id: form_id,
            width: Pt::new(width),
            height: Pt::new(height),
        })
    }

    pub fn draw_svg(
        &mut self,
        svg: SvgResource,
        x: Pt,
        y: Pt,
        width: Pt,
        height: Pt,
    ) -> Result<(), AppError> {
        if width.value() <= 0.0 || height.value() <= 0.0 {
            return Err(AppError::PdfWriter(
                "SVG width and height must be greater than zero".to_string(),
            ));
        }

        if svg.width.value() <= 0.0 || svg.height.value() <= 0.0 {
            return Err(AppError::PdfWriter(
                "SVG has invalid intrinsic dimensions".to_string(),
            ));
        }

        let name = format!("SVG{}", svg.id.0);

        self.document
            .add_xobject(self.page_id, name.as_bytes().to_vec(), svg.id)
            .map_err(|error| AppError::PdfWriter(format!("failed to add SVG XObject: {error}")))?;

        let pdf_y = self.page_height.value() - y.value() - height.value();

        let scale_x = width.value() / svg.width.value();

        let scale_y = height.value() / svg.height.value();

        self.operations.push(Operation::new("q", vec![]));

        self.operations.push(Operation::new(
            "cm",
            vec![
                scale_x.into(),
                0.0.into(),
                0.0.into(),
                scale_y.into(),
                x.value().into(),
                pdf_y.into(),
            ],
        ));

        self.operations
            .push(Operation::new("Do", vec![Object::Name(name.into_bytes())]));

        self.operations.push(Operation::new("Q", vec![]));

        Ok(())
    }

    pub fn draw_svg_string(
        &mut self,
        svg: &str,
        x: Pt,
        y: Pt,
        width: Pt,
        height: Pt,
    ) -> Result<(), AppError> {
        let resource = self.embed_svg(svg)?;

        self.draw_svg(resource, x, y, width, height)
    }
}

fn collect_referenced_objects(
    source: &Document,
    object: &Object,
    output: &mut BTreeSet<ObjectId>,
) -> Result<(), AppError> {
    match object {
        Object::Reference(id) => {
            if !output.insert(*id) {
                return Ok(());
            }

            let referenced = source.objects.get(id).ok_or_else(|| {
                AppError::PdfWriter(format!("referenced PDF object {:?} does not exist", id))
            })?;

            collect_referenced_objects(source, referenced, output)?;
        }

        Object::Array(objects) => {
            for object in objects {
                collect_referenced_objects(source, object, output)?;
            }
        }

        Object::Dictionary(dictionary) => {
            for (_, value) in dictionary.iter() {
                collect_referenced_objects(source, value, output)?;
            }
        }

        Object::Stream(stream) => {
            for (_, value) in stream.dict.iter() {
                collect_referenced_objects(source, value, output)?;
            }
        }

        _ => {}
    }

    Ok(())
}
