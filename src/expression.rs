#[derive(Debug, PartialEq)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64)
}