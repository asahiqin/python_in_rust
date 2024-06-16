#[derive(Debug)]
pub struct ASTNode{
    pub(crate) body: Box<[ASTType]>,
    pub(crate) lineno:usize,
    pub(crate) end_lineno:usize,
    pub(crate) col_offset:usize,
    pub(crate) end_col_offset:usize
}
#[derive(Debug)]
pub enum ASTType{
    ASTAssign(ASTAssign),
}
#[derive(Debug)]
pub enum ASTAssignSubNode{
    ASTName(ASTName),
    ASTConstant(ASTConstant)
}
#[derive(Debug)]
pub struct ASTAssign{
    pub(crate) target:ASTName,
    pub(crate) value:ASTAssignSubNode,
    pub(crate) type_comment:String
}
#[derive(Debug)]
pub struct ASTName{
    pub(crate) id:String,
    pub(crate) value:ASTConstant,
    pub(crate) type_comment:String
}
#[derive(Debug)]
pub enum DataType{
    Int(isize),
    Float(f64),
    String(String),
    List(Box<[DataType]>),
    None
}
#[derive(Debug)]
pub struct ASTConstant{
    pub(crate) value:DataType,
    pub(crate) type_comment:String
}