use crate::ast::ast_struct::DataType;

#[derive(Debug,Clone,PartialEq)]
pub struct PyObject{
    data_type: DataType
}

pub fn obj_bool(x:bool) -> PyObject{
    PyObject{
        data_type:DataType::Bool(x)
    }
}

pub fn obj_str(x:String) -> PyObject{
    PyObject{
        data_type:DataType::Str(x)
    }
}

pub fn obj_int(x:i64) -> PyObject{
    PyObject{
        data_type:DataType::Int(x)
    }
}

pub fn obj_float(x:f64) -> PyObject{
    PyObject{
        data_type:DataType::Float(x)
    }
}