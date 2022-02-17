#[derive(Debug)]
pub enum Token {
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Comma,
    String(String),
	Number(f64),
	Boolean(bool),
	Null
}
