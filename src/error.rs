#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    FontError(String),
    HtmlParsing(String),
    CssParsing(String),
    Layout(String),
    PdfWriter(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::FontError(error) => write!(f, "font error: {error}"),
            Self::HtmlParsing(error) => write!(f, "HTML parsing error: {error}"),
            Self::CssParsing(error) => write!(f, "CSS parsing error: {error}"),
            Self::Layout(error) => write!(f, "layout error: {error}"),
            Self::PdfWriter(error) => write!(f, "PDF writer error: {error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
