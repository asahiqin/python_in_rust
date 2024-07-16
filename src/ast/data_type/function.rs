use std::collections::HashMap;

use crate::ast::data_type::class::Class;

#[derive(Debug,Clone)]
enum PyRecommendType<T>
where T: Fn(HashMap<String, Class<T>>) -> Class<T>{
    None,
    Class(Box<Class<T>>)
}

#[derive(Debug,Clone)]
pub struct Function<T>
where T: Fn(HashMap<String, Class<T>>) -> Box<Class<T>>
{
    pub args: HashMap<String,PyRecommendType<T>>,
    recommend_result_type: PyRecommendType<T>,
    def: T
}

impl<T> Default for Function<T>
where T: Fn(HashMap<String, Class<T>>) -> Box<Class<T>>{
    fn default() -> Self {
        fn not_impl(x: HashMap<String, Class<T>>) -> Box<Class<T>>
        {
            panic!("not impl")
        }
        Function{
            args: vec![].into_iter().collect(),
            recommend_result_type: PyRecommendType::None,
            def: not_impl,
        }
    }
}
impl<T> Function<T>
where T: Fn(HashMap<String, Class<T>>) -> Box<Class<T>> {
    fn args<U>(&self, args: U) -> Function<T>
    where U: IntoIterator<Item = (String,PyRecommendType<T>)>
    {
        let args:HashMap<String,PyRecommendType<T>> = args.into_iter().collect();
        Function{
            args,
            ..self.clone()
        }
    }
}