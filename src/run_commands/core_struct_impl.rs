use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::ast_struct::{Assign, BinOp, Compare, Constant, Name, Operator, PyCtx, PyRootNode, Type};
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::ast::scanner::build_scanner;
use crate::object::object::{obj_bool, PyObject};

pub trait Calc {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant;
}
fn deref_expression(data: Type, env: &mut PyNamespace, namespace: Namespace) -> Constant {
    let mut _x: Constant;
    match data {
        Type::Constant(x) => {
            _x = x.clone();
        }
        Type::Name(mut x) => {
            _x = x.exec(env, namespace);
        }
        Type::BinOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::Compare(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::UnaryOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::BoolOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        _ => panic!("Error at calc"),
    }
    _x
}
impl PyRootNode {
    pub fn exec(&mut self,env:&mut PyNamespace) -> Type {
        exec_commands(&self.body, env, Namespace::Global)
    }
    pub fn parser(&mut self, s: String) {
        let mut scanner = build_scanner(s);
        scanner.scan();
        let mut parser = build_parser(scanner, PyNamespace::default());
        self.body = parser.create_vec()
    }
}
pub fn exec_commands(
    command: &Vec<Box<Type>>,
    namespace: &mut PyNamespace,
    current_namespace: Namespace,
) -> Type {
    for (index, item) in command.iter().enumerate() {
        match item.clone().exec(namespace, current_namespace.clone()) {
            Type::None => {}
            Type::Constant(x) => {
                if index + 1 == command.len() {
                    return Type::Constant(x);
                }
            }
            Type::Break => return Type::Break,
            Type::Continue => return Type::Continue,
            _ => {}
        }
    }
    Type::None
}

impl Type {
    pub fn exec(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Type {
        match self {
            Type::Assign(x) => x.exec(env, current_namespace),
            Type::Constant(x) => Type::Constant(x.clone()),
            Type::Name(_) => {
                todo!()
            }
            Type::Attribute(_) => {
                todo!()
            }
            Type::BinOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::Compare(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::UnaryOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::BoolOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::Print(x) => {
                println!(
                    "{}",
                    todo!()
                );
                Type::None
            }
            Type::If(x) => x.exec(env, current_namespace),
            Type::While(x) =>  x.exec(env, current_namespace),
            Type::Break => Type::Break,
            Type::Continue => Type::Continue,
            Type::None => Type::None,
        }
    }
}

impl Assign {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        match *self.target.clone() {
            Type::Name(x) => match x.ctx {
                PyCtx::Store => {
                    let mut value = deref_expression(*self.value.clone(), env, namespace.clone());
                    match namespace {
                        Namespace::Builtin => {
                            panic!("You cannot set built variable in code")
                        }
                        Namespace::Global => {
                            env.set_global(x.id, &mut value.value);
                        }
                        _ => todo!(),
                    }
                }
                _ => panic!("Error to store name:{}", x.id),
            },
            _ => todo!(),
        }
        Type::None
    }
}
impl Name {
    pub fn ctx(&mut self, ctx: PyCtx) -> Self {
        self.ctx = ctx;
        return self.clone();
    }
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Constant {
        match namespace {
            Namespace::Builtin => match env.get_builtin(self.id.clone()) {
                Ok(x) => return Constant::new(x.clone()),
                _ => {}
            },
            Namespace::Global => match env.get_global(self.id.clone()) {
                Ok(x) => return Constant::new(x.clone()),
                _ => {}
            },
            Namespace::Enclosing(x) => match env.get_enclosing(x, self.id.clone()) {
                Ok(x) => return Constant::new(x.clone()),
                _ => {}
            },
            Namespace::Local(_, ..) => {
                todo!()
            }
        }
        match env.get_builtin(self.id.clone()) {
            Ok(x) => {
                return Constant::new(x.clone())
            }
            Err(x) => {
                panic!("{}", x)
            }
        }
    }
}
impl Calc for BinOp {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant {
        let mut x: PyObject = deref_expression(*self.left.clone(), env, current_namespace.clone())
            .clone()
            .value;
        let y: PyObject = deref_expression(*self.right.clone(), env, current_namespace.clone())
            .clone()
            .value;
        todo!()
    }
}

impl Compare {
    fn compare(operator: Operator, mut left: PyObject, right: PyObject,namespace: Namespace, env: &mut PyNamespace) -> bool {
        todo!()
    }

    fn compare_calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> bool {
        let mut comparators = vec![*self.left.clone()];
        comparators.extend(*self.comparators.clone());
        for (index, left) in comparators.iter().enumerate() {
            let left = deref_expression(left.clone(), env, current_namespace.clone());
            if index + 1 == comparators.len() {
                return true;
            }
            let right = deref_expression(
                comparators[index + 1].clone(),
                env,
                current_namespace.clone(),
            );
            if !Self::compare(self.ops[index].clone(), left.value, right.value, current_namespace.clone(), env) {
                return false;
            }
        }
        true
    }
}
impl Calc for Compare {
    fn calc(&mut self) -> Constant {
        todo!()
    }
}
impl Calc for UnaryOp {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant {
        let mut x: PyObject = deref_expression(*self.operand.clone(), env, current_namespace.clone())
            .clone()
            .value;
        todo!()
    }
}

impl Calc for BoolOp {
    fn calc(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Constant {
        match self.op {
            Operator::And => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i, env, namespace.clone());
                    if !obj_to_bool(i_constant.value,namespace.clone(), env) {
                        return Constant::new(obj_bool(false));
                    }
                }
                return Constant::new(obj_bool(true));
            }
            Operator::Or => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i, env, namespace.clone());
                    if obj_to_bool(i_constant.value, namespace.clone(), env) {
                        return Constant::new(obj_bool(true));
                    }
                }
                return Constant::new(obj_bool(false));
            }
            _ => panic!("Unsupported Bool Operate"),
        }
    }
}

impl If {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        let test = deref_expression(*self.test.clone(), env, namespace.clone());
        return if obj_to_bool(test.value, namespace.clone(), env) {
            exec_commands(&self.body.clone(), env, namespace.clone())
        } else {
            exec_commands(&self.orelse.clone(), env, namespace.clone())
        }
    }
}
impl While {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        let mut test = deref_expression(*self.test.clone(), env, namespace.clone());
        let mut break_line=true;
        while obj_to_bool(test.value.clone(), namespace.clone(), env) {
            test = deref_expression(*self.test.clone(), env, namespace.clone());
            match exec_commands(&self.body,env,namespace.clone()){
                Type::Break => {
                    break_line = false;
                    break
                }
                Type::Continue => {
                    continue
                }
                _ => {}
            }
        }
        if break_line{
            exec_commands(&self.orelse, env, namespace.clone());
        }
        Type::None
    }
}