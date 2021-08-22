pub enum MalType {
    Number(i32),
}

pub type AST = Vec<MalType>;
pub type BoxedError = Box<dyn std::error::Error>;
