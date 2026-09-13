use crate::{
    font::{loader::Font, shaper::Shaper},
    units::{Direction, Pt},
};

fn load_font() -> Font {
    Font::get_font("B-Nazanin").unwrap()
}
#[test]
fn font_file_can_be_loaded() {
    let font = load_font();
    assert_eq!(font.family, "B-Nazanin");
    assert!(!font.data.is_empty());
}
#[test]
fn font_contains_persian() {
    let font = load_font();
    let face = rustybuzz::Face::from_slice(font.data, 0).expect("invalid font");

    assert!(face.glyph_index('س').is_some());
    assert!(face.glyph_index('ل').is_some());
    assert!(face.glyph_index('ا').is_some());
    assert!(face.glyph_index('م').is_some());
}

#[test]
fn reads_units_per_em() {
    let font = load_font();
    let units = font.units_per_em().expect("failed to read unit_per_em()");
    println!("units per em ={units}");
    assert!(units > 0)
}

fn test_font() -> Font {
    Font::get_font("B-Nazanin").unwrap()
}

#[test]
fn shape_returns_glyphs() {
    let font = test_font();
    let text = "سلام دنیا";

    let glyphs = Shaper::shape_glyphs(&font, text, Direction::RTL).expect("failed to shape text");

    assert!(!glyphs.is_empty());
}

#[test]
fn shaped_text_contains_original_text() {
    let font = test_font();
    let text = "سلام دنیا";

    let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
        .expect("failed to shape the text");

    assert_eq!(shaped_text.text, text);
}

#[test]
fn shaped_text_has_glyphs() {
    let font = test_font();
    let text = "سلام دنیا";

    let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
        .expect("failed to shape the text");

    assert!(!shaped_text.glyphs.is_empty());
}

#[test]
fn shaped_text_has_positive_width() {
    let font = test_font();
    let text = "سلام دنیا";

    let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
        .expect("failed to shape the text");

    assert!(shaped_text.width.value() > 0.0);
}
