#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    FontError(String),
}
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}
