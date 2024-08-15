pub struct BuiltInFunction {
    pub obj: String,
    pub method: String,
}

#[macro_export]
macro_rules! define_builtin_function {
    /// func: Callable object name
    /// method: The Call Method's Name
    /// obj_id: The Object's ID
    (func:$name:ident;method:$pattern:expr;obj_id:$obj:expr) => {
        |id:String,method:String,env:&mut PyNamespace,namespace:Namespace,data_type:Vec<DataType>| -> Option<PyResult>{
            if $pattern == method && $obj == id  {
                return Some($name(env,namespace,data_type))
            }else{
                return None;
            }
        }
    };
    (pattern:{$($match_pattern:expr)*}) => {
        use crate::object::namespace::{PyNamespace,Namespace};
        use crate::ast::ast_struct::DataType;
        use crate::object::object::PyResult;
        use crate::object::define_builtin_function::BuiltInFunction;
        impl BuiltInFunction{
            pub fn exec(&mut self,env:&mut PyNamespace,namespace:Namespace,data_type:Vec<DataType>) -> PyResult{
                $( match $match_pattern(self.obj.clone(),self.method.clone(),env, namespace.clone(), data_type.clone()){
                    Some(x) => return x,
                    None => {}
                }; )*
                PyResult::None
            }
        }
    };
}
