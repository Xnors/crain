use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {source}")]
    IO { #[from] source: std::io::Error },
    
    #[error("Mismatched bracket: '{bracket}' at {line}:{row}")]
    MismatchedBracket { bracket: char, line: usize, row: usize },
}

