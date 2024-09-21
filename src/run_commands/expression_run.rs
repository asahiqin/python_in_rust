use uuid::Uuid;
use crate::ast::ast_struct::{BinOp, Constant, Operator, Print, Type};
use crate::error::ErrorType;
use crate::object::define_builtin_function::BuiltinFunctionArgs;
use crate::object::namespace::PyVariable;
use crate::run_commands::RunAble;

impl RunAble for Type{
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs) -> Option<Result<Uuid, ErrorType>> {
        match self {
            Type::Constant(x) => {x.clone().run(builtin_function_args)}
            Type::Print(x) => {x.clone().run(builtin_function_args)}
            Type::BinOp(x) => {x.clone().run(builtin_function_args)}
            _ => {todo!()}
        }
    }
}
impl RunAble for Constant{
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs) -> Option<Result<Uuid, ErrorType>> {
        Some(Ok(builtin_function_args.env.variable_pool.store_new_value(self.value.clone())))
    }
}
macro_rules! calc_marco {
    (left:$left:ident,left_uuid:$left_uuid:ident, right:$right:ident, right_uuid:$right_uuid:ident,
    method:$method:expr, rmethod:$rmethod:expr,
    call:$call:ident, builtin_function_args:$builtin_function_args:ident) => {
        match $left {
            PyVariable::Object(mut x) => {
                PyVariable::from(x.call($method.parse().unwrap(), vec![$right_uuid], $builtin_function_args))
            }
            PyVariable::DataType(x) => {
                match $right {
                    PyVariable::Object(mut y) => {
                        PyVariable::from(y.call($rmethod.parse().unwrap(), vec![$left_uuid], $builtin_function_args))
                    }
                    PyVariable::DataType(y) => {
                        PyVariable::DataType(x.$call(y).unwrap())
                    }
                }
            }
        }
    };
}
impl RunAble for BinOp{
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs) -> Option<Result<Uuid, ErrorType>> {
        let left_uuid = self.left.clone().run(builtin_function_args).unwrap().unwrap();
        let right_uuid = self.right.clone().run(builtin_function_args).unwrap().unwrap();
        let left_value = builtin_function_args.env.variable_pool.get_value(left_uuid.clone()).unwrap();
        let right_value = builtin_function_args.env.variable_pool.get_value(right_uuid.clone()).unwrap();
        let result:PyVariable = match self.op {
            Operator::Add => {
                calc_marco!(left:left_value, left_uuid:left_uuid, right:right_value, right_uuid:right_uuid,
                    method:"__add__", rmethod:"__radd__", call:add, builtin_function_args:builtin_function_args)
            }
            Operator::Sub => {
                calc_marco!(left:left_value, left_uuid:left_uuid, right:right_value, right_uuid:right_uuid,
                    method:"__sub__", rmethod:"__rsub__", call:sub, builtin_function_args:builtin_function_args)
            }
            Operator::Mult => {
                calc_marco!(left:left_value, left_uuid:left_uuid, right:right_value, right_uuid:right_uuid,
                    method:"__mul__", rmethod:"__rmul__", call:mul, builtin_function_args:builtin_function_args)
            }
            Operator::Div => {
                calc_marco!(left:left_value, left_uuid:left_uuid, right:right_value, right_uuid:right_uuid,
                    method:"__div__", rmethod:"__rdiv__", call:div, builtin_function_args:builtin_function_args)
            }
            Operator::Mod => {todo!()}
            Operator::Pow => {todo!()}
            _ => {todo!()}
        };
        Some(Ok(builtin_function_args.env.variable_pool.store_new_value(result)))
    }
}
impl RunAble for Print{
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs) -> Option<Result<Uuid, ErrorType>> {
        let uuid =self.arg.run(builtin_function_args).unwrap().unwrap();
        let value =builtin_function_args.env.variable_pool.get_value(uuid.clone());
        println!("{:?}", value);
        None
    }
}
