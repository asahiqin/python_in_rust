use crate::object::builtin::py_type::builtin_method_or_function;
use crate::object::object::PyFunction;
/// 定义对象的函数调用的宏
#[macro_export]
macro_rules! def {
    () => {};
    (def $ident:ident with $name:expr;$($tail:tt)*)=>{
        pub fn $ident(
            &mut self,
            args: Vec<Uuid>,
            builtin_function_args: &mut BuiltinFunctionArgs
        ) -> PyResult {
            self.call($name.to_string(), args, builtin_function_args)
        }
        def!($($tail)*);
    };
    (to $obj:ident;$($tail:tt)*) => {
        impl $obj{
            def!($($tail)*);
        }
    }
}

#[macro_export]
macro_rules! def_class {
    (obj:$obj:ident;
    env:$env:ident;
    builtin:$builtin:ident;
    name:$name:expr;
    args:$args:expr;
    method:$method:expr;
    func:$func:expr) => {
        $obj.set_attr(
            $method,
            builtin_method_or_function(
                PyFunction::default()
                .run_default($method)
                .arg($args),
                $env
            ).into(),
            $env
        );
        $builtin.define_obj(
            $name,
            $method,
            Box::new($func),
        )
    };
}