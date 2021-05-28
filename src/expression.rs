use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
}
