use std::string::ToString;

#[derive(Debug)]
pub struct ASTNode {
    pub(crate) body: Vec<ASTType>,
    pub(crate) lineno: usize,
    pub(crate) end_lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_col_offset: usize,
}
#[derive(Debug)]
pub enum ASTType {
    ASTAssign(ASTAssign),
}
#[derive(Debug)]
pub enum ASTAssignSubNode {
    ASTName(ASTName),
    ASTConstant(ASTConstant),
}
#[derive(Debug)]
pub struct ASTAssign {
    pub(crate) target: ASTName,
    pub(crate) value: ASTAssignSubNode,
    pub(crate) type_comment: &'static str,
}
#[derive(Debug)]
pub struct ASTName {
    pub(crate) id: &'static str,
    pub(crate) value: ASTConstant,
    pub(crate) type_comment: &'static str,
}
#[derive(Debug)]
pub enum DataType {
    Int(isize),
    Float(f64),
    String(String),
    List(Vec<DataType>),
    None,
}
#[derive(Debug)]
pub struct ASTConstant {
    pub(crate) value: DataType,
    pub(crate) type_comment: &'static str,
}
