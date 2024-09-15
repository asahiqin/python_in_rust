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
