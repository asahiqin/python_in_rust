#[macro_export]
macro_rules! define_stmt {
    // 定义语句
    (stmt_id:$id:ident;ast:$ast:ident;{$($tt:tt)*};$($args:tt)*) => {
        impl Parser{
            pub fn $id(&mut self) -> Result<Type, ErrorType>{
                let mut parser:&mut Parser=self;
                let mut ast:$ast = $ast::builder().build();
                let res:bool = $crate::define_rules!(_p:parser;_a:ast;$($tt)*);
                if res {
                    Ok(define_stmt!(_a:$ast;_v:ast;$($args)*))
                } else {
                    Ok(Type::None)
                }
            }
        }
    };
    // 假如语句需要用box包装
    (_a:$ast:ident;_v:$var:ident;with_box) => {
        Type::$ast(Box::new($var.clone()))
    };
    (_a:$ast:ident;variable:$var:ident) => {
        Type::$ast($var.clone())
    }
}
#[macro_export]
macro_rules! define_rules {
    // 定义token规则
    (_p:$parser:expr;_a:$ast:expr;<token:($token:expr)>;$($tt:tt)*) => {
        {
            let mut parser:&mut Parser=&mut $parser;
            if parser.token_iter.catch([$token]) {
                define_rules!(_p:parser;_a:$ast;$($tt)*)
            }else {
                false
            }
        }
    };
    (_p:$parser:expr;_a:$ast:expr;<expr:($args:ident)>;$($tt:tt)*) => {
        {
            let mut parser:&mut Parser=&mut $parser;
            let ast=&mut $ast;
            ast.$args = Box::new(parser.expression()?);
            define_rules!(_p:parser;_a:ast;$($tt)*)
        }
    };
    (_p:$parser:expr;_a:$ast:expr;<end_line>) => {
        {
            let mut parser:&mut Parser=&mut $parser;
            parser.token_iter.consume(TokenType::LineBreak, "".to_string())?;
            true
        }
    }
}
