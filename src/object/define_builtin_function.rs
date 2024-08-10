use crate::ast::ast_struct::DataType;
use crate::object::namespace::{Namespace, PyNamespace};
use crate::object::object::PyResult;

pub struct BuiltInFunction{
    pub method:String
}

#[macro_export]
macro_rules! define_builtin_function {
    (func:$name:ident;value:$pattern:expr) => {
        |method:String,env:&mut PyNamespace,namespace:Namespace,data_type:Vec<DataType>| -> Option<PyResult>{
            if $pattern == method {
                return Some($name(env,namespace,data_type))
            }else{
                return None;
            }
        }
    };
    (pattern:{$($match_pattern:expr)*}) => {
        impl BuiltInFunction{
            pub fn exec(&mut self,env:&mut PyNamespace,namespace:Namespace,data_type:Vec<DataType>) -> PyResult{
                $( match $match_pattern(self.method.clone(),env, namespace.clone(), data_type.clone()){
                    Some(x) => return x,
                    None => {}
                }; )*
                PyResult::None
            }
        }
    };
}
