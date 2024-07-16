use std::collections::HashMap;
use std::fmt::Debug;

// use crate::ast::data_type::class::Class;



/*
#[derive(Clone)]
pub enum PyRecommendType<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    None,
    Class(Class<T>)
}
#[derive(Clone)]
pub enum PyRustFunction<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    None,
    Def(T)
}

#[derive(Clone)]
pub struct Function<T>
where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    pub args: HashMap<String,PyRecommendType<T>>,
    pub recommend_result_type: Box<PyRecommendType<T>>,
    pub def: PyRustFunction<T>
}

impl<T> Function<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    pub(crate) fn default() -> Box<Function<T>>
    {

        let args:HashMap<String,PyRecommendType<T>> = vec![].into_iter().collect();
        Box::from(Function{
            args,
            recommend_result_type:Box::from(PyRecommendType::None),
            def: PyRustFunction::None
        })
    }
    pub(crate) fn build<U>(args: U, recommend_result_type:Box<PyRecommendType<T>>, def: T) -> Box<Function<T>>
    where U: IntoIterator<Item = (String,PyRecommendType<T>)>
    {
        let args:HashMap<String,PyRecommendType<T>> = args.into_iter().collect();
        Box::from(Function{
            args,
            recommend_result_type,
            def: PyRustFunction::Def(def)
        })
    }
}
*/