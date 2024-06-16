use crate::ast::ast_struct::{ASTAssign, ASTAssignSubNode, ASTConstant, ASTName, ASTNode, ASTType, DataType};

pub fn tokenize(code: String) -> ASTNode{
    ASTNode{
        body: Box::from([ASTType::ASTAssign(ASTAssign {
            target: ASTName {
                id: "a".to_string(),
                value: ASTConstant { value: DataType::None, type_comment: "".to_string() },
                type_comment: "".to_string(),
            },
            value: ASTAssignSubNode::ASTConstant(ASTConstant { value: DataType::Int(0), type_comment: "".to_string() }),
            type_comment: "".to_string(),
        })]),
        lineno: 1,
        end_lineno: 1,
        col_offset: 0,
        end_col_offset: 0,
    }
}

