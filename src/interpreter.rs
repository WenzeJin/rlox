
use crate::ast::{expr, stmt};
use crate::value::{LoxFunction, LoxValue};
use crate::class::{LoxClass, LoxInstance};
use crate::env::Environment;
use crate::ast::token::{Token, TokenType};
use crate::error::RloxError;
use crate::builtin::regist_builtins;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct Interpreter {
    pub had_error: bool,
    pub env: Environment,
    pub locals: HashMap<Token, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut environment = Environment::new();
        regist_builtins(&mut environment);
        Interpreter {
            had_error: false,
            env: environment,
            locals: HashMap::new(),
        }
    }

    pub fn change_env(&mut self, env: Environment) -> Environment {
        std::mem::replace(&mut self.env, env)
    }

    
    /// Resolves a variable by associating it with a specific depth in the environment.
    ///
    /// This function is used during the static analysis phase to record the depth
    /// at which a variable is located in the environment. The depth is stored in
    /// the `locals` map, which is later used during runtime to efficiently retrieve
    /// the variable's value.
    ///
    /// # Parameters
    /// - `name`: The `Token` representing the variable's name.
    /// - `depth`: The depth of the variable in the environment hierarchy.
    pub fn resolve(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }

    /// If the program is a valid program, it will be interpreted. <br>
    /// If you want to interpret and handle any error that may occur, use this function. <br>
    /// Otherwise, use visitor pattern, which means stmt.accept(self) <br>
    pub fn interpret(&mut self, program: stmt::Stmt) {
        self.had_error = false;
        if let stmt::Stmt::Program(_) = program {
            if let Err(e) = program.accept(self) {
                self.runtime_error(e);
            }
        } else {
            println!("Input is not a valid program!");
            self.had_error = true;
        }
    }

    /// This function is used to execute a block of statements. <br>
    /// Different from the visit_block_stmt function, this function does not enter a new scope. <br>
    /// So it is used to execute a block of statements in the current scope. <br>
    pub fn execute_block(&mut self, block: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        for statement in block {
            statement.accept(self)?;
        }
        Ok(())
    }
}

impl Interpreter {
    fn is_truthy(value: &LoxValue) -> bool {
        match value {
            LoxValue::Boolean(b) => *b,
            LoxValue::Null => false,
            _ => true,
        }
    }

    fn runtime_error(&mut self, error: RloxError) {
        println!("{}", error);
    }
}

// MARK: Expression Visitor

impl expr::Visitor<Result<LoxValue, RloxError>> for Interpreter {
    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let lv = left.accept(self)?;
        let rv = right.accept(self)?;

        match operator.t_type {
            TokenType::Plus => {
                match (lv, rv) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                    (LoxValue::String(l), LoxValue::String(r)) => Ok(LoxValue::String(format!("{l}{r}"))),
                    _ => Err(RloxError::RuntimeError("Operands must be two numbers or two strings.".to_string()))
                }
            }
            TokenType::Minus => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Number(l - r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::Star => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Number(l * r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::Slash => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    if r == 0.0 {
                        Err(RloxError::RuntimeError("Division by zero.".to_string()))
                    } else {
                        Ok(LoxValue::Number(l / r))
                    }
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::Greater => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l > r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::GreaterEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l >= r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::Less => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l < r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::LessEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l <= r))
                } else {
                    Err(RloxError::RuntimeError("Operands must be two numbers.".to_string()))
                }
            }
            TokenType::EqualEqual => 
                Ok(LoxValue::Boolean(lv == rv)),
            TokenType::BangEqual => 
                Ok(LoxValue::Boolean(lv != rv)),
            _ => Err(RloxError::RuntimeError("Unknown binary operator.".to_string()))
        }
    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let lv = left.accept(self)?;
        match operator.t_type {
            TokenType::Or => {
                if Interpreter::is_truthy(&lv) {
                    Ok(lv)
                } else {
                    right.accept(self)
                }
            }
            TokenType::And => {
                if Interpreter::is_truthy(&lv) {
                    right.accept(self)
                } else {
                    Ok(lv)
                }
            }
            _ => Err(RloxError::RuntimeError("Unknown logical operator.".to_string()))
        }
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<LoxValue, RloxError> {
        expression.accept(self)
    }

    fn visit_literal_expr(&mut self, value: &expr::LiteralValue) -> Result<LoxValue, RloxError> {
        match value {
            expr::LiteralValue::Number(n) => Ok(LoxValue::Number(*n)),
            expr::LiteralValue::String(s) => Ok(LoxValue::String(s.clone())),
            expr::LiteralValue::Boolean(b) => Ok(LoxValue::Boolean(*b)),
            expr::LiteralValue::Nil => Ok(LoxValue::Null),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let rv = right.accept(self)?;
        match operator.t_type {
            TokenType::Minus => {
                if let LoxValue::Number(n) = rv {
                    Ok(LoxValue::Number(-n))
                } else {
                    Err(RloxError::RuntimeError("Operand must be a number.".to_string()))
                }
            }
            TokenType::Bang => {
                Ok(LoxValue::Boolean(!Interpreter::is_truthy(&rv)))
            }
            _ => Err(RloxError::RuntimeError("Unknown unary operator.".to_string())),
        }
        
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<LoxValue, RloxError> {
        if let Some(depth) = self.locals.get(name) {
            self.env.get_by_depth(name, *depth)
        } else {
            self.env.get(name)
        }
    }

    fn visit_assign_expr(&mut self, left: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let value = right.accept(self)?;
        if let Some(depth) = self.locals.get(left) {
            self.env.assign_by_depth(left, value.clone(), *depth)?;
            Ok(value)
        } else {
            self.env.assign(left, value.clone())?;
            Ok(value)
        }
    }

    fn visit_call_expr(&mut self, callee: &expr::Expr, arguments: &[expr::Expr]) -> Result<LoxValue, RloxError> {
        let callee_value = callee.accept(self)?;
        if let LoxValue::Callable(method) = callee_value {
            let mut arg_values = Vec::new();
            for arg in arguments {
                arg_values.push(arg.accept(self)?);
            }
            if arg_values.len() != method.arity() as usize {
                return Err(RloxError::RuntimeError(format!("Expected {} arguments but got {}.", method.arity(), arg_values.len())));
            }
            method.invoke(self, arg_values)
        } else if let LoxValue::Class(class) = callee_value {
            let instance = LoxInstance::new(&class);
            let class = instance.class.clone();
            let initializer = class.borrow().find_method("init");
            let instance_rc = Rc::new(RefCell::new(instance));
            if let Some(initializer) = initializer {
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(arg.accept(self)?);
                }
                if arg_values.len() != initializer.arity() as usize {
                    return Err(RloxError::RuntimeError(format!("Expected {} arguments but got {}.", initializer.arity(), arg_values.len())));
                }
                
                initializer.bind(Rc::clone(&instance_rc)).invoke(self, arg_values)?;
            }

            Ok(LoxValue::Instance(Rc::clone(&instance_rc)))
        } else {
            Err(RloxError::RuntimeError("Can only call functions and classes.".to_string()))
        }
    }

    fn visit_get_expr(&mut self, object: &expr::Expr, name: &Token) -> Result<LoxValue, RloxError> {
        let object_value = object.accept(self)?;
        if let LoxValue::Instance(instance) = object_value {
            instance.borrow().get(&name.lexeme, &instance)
        } else {
            Err(RloxError::RuntimeError("Only instances have properties.".to_string()))
        }
    }

    fn visit_set_expr(&mut self, object: &expr::Expr, name: &Token, value: &expr::Expr) -> Result<LoxValue, RloxError> {
        let object_value = object.accept(self)?;
        if let LoxValue::Instance(instance) = object_value {
            let value = value.accept(self)?;
            instance.borrow_mut().set(&name.lexeme, value.clone());
            Ok(value)
        } else {
            Err(RloxError::RuntimeError("Only instances have fields.".to_string()))
        }
    }

    fn visit_this_expr(&mut self, name: &Token) -> Result<LoxValue, RloxError> {
        if let Some(depth) = self.locals.get(name) {
            self.env.get_by_depth(name, *depth)
        } else {
            self.env.get(name)
        }
    }

    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> Result<LoxValue, RloxError> {
        if let Some(depth) = self.locals.get(keyword) {
            let super_class = self.env.get_by_depth(keyword, *depth)?;
            if let LoxValue::Class(super_class) = super_class {
                if let Some(method) = super_class.borrow().find_method(&method.lexeme) {
                    let this_token = Token::new(TokenType::This, "this".to_string(), 0);
                    let this_value = self.env.get_by_depth(&this_token, *depth - 1)?;
                    if let LoxValue::Instance(instance) = this_value {
                        return Ok(LoxValue::Callable(method.bind(Rc::clone(&instance))));
                    } else {
                        unreachable!("This should always be an instance.");
                    }
                } else {
                    return Err(RloxError::RuntimeError(format!("Undefined property '{}'.", method.lexeme)));
                }
            } else {
                unreachable!("Super class should always be a class.");
            }
        } else {
            return Err(RloxError::RuntimeError("Can't use 'super' in a class with no superclass.".to_string()));
        }
    }
}


// MARK: Statement Visitor

impl stmt::Visitor<Result<(), RloxError>> for Interpreter {

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> Result<(), RloxError> {
        expression.accept(self)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> Result<(), RloxError> {
        let value = expression.accept(self)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_program_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        for statement in statements {
            statement.accept(self)?;
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        self.env.enter_scope();
        for statement in statements {
            if let Err(e) = statement.accept(self) {
                self.env.exit_scope();
                return Err(e);
            }
        }
        self.env.exit_scope();
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<expr::Expr>) -> Result<(), RloxError> {
        let value = if let Some(expr) = initializer {
            expr.accept(self)?
        } else {
            LoxValue::Null
        };
        self.env.define(&name.lexeme, value);
        Ok(())
    }

    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &Box<stmt::Stmt>, else_branch: &Option<Box<stmt::Stmt>>) -> Result<(), RloxError> {
        if Interpreter::is_truthy(&condition.accept(self)?) {
            then_branch.accept(self)?;
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &Box<stmt::Stmt>) -> Result<(), RloxError> {
        while Interpreter::is_truthy(&condition.accept(self)?) {
            body.accept(self)?;
        }
        Ok(())
    }

    fn visit_function_decl_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Rc<Vec<stmt::Stmt>>) -> Result<(), RloxError> {
        // resolve function name
        let name = name.lexeme.clone();
        // resolve names of parameters
        let params: Vec<String> = params.iter().map(|param| param.lexeme.clone()).collect();
        // create a new function
        let function = LoxValue::Callable(LoxFunction::UserFunction{
            def_name: name.clone(),
            params,
            body: Rc::clone(body),
            closure: Rc::clone(&self.env.values),
            is_initializer: false,
        });
        // define the function in the current environment
        self.env.define(&name, function);
        Ok(())
    }

    fn visit_return_stmt(&mut self, value: &Option<expr::Expr>) -> Result<(), RloxError> {
        let value = if let Some(expr) = value {
            expr.accept(self)?
        } else {
            LoxValue::Null
        };
        Err(RloxError::ReturnValue(value))
    }

    fn visit_class_decl_stmt(&mut self, name: &Token, maybe_super_class: &Option<expr::Expr>, methods: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        let class_name = name.lexeme.clone();
        self.env.define(&class_name, LoxValue::Null);

        let mut class = LoxClass::new(class_name.clone());

        // set super class
        if let Some(super_class) = maybe_super_class {
            if let LoxValue::Class(super_class) = super_class.accept(self)? {
                class.super_class = Some(Rc::clone(&super_class));
                // define super
                self.env.enter_scope();
                self.env.define("super", LoxValue::Class(Rc::clone(&super_class)));
            } else {
                return Err(RloxError::SemanticError("Superclass must be a class".to_string()));
            }
        }

        for method in methods {
            if let stmt::Stmt::FunctionDecl(name, params, body) = method {
                let method_name = name.lexeme.clone();
                let function = LoxFunction::UserFunction{
                    def_name: method_name.clone(),
                    params: params.iter().map(|param| param.lexeme.clone()).collect(),
                    body: Rc::clone(body),
                    closure: Rc::clone(&self.env.values),
                    is_initializer: method_name == "init",
                };
                // eprintln!("clousure: {:?}", Rc::clone(&self.env.values));
                class.methods.insert(method_name.clone(), function);
            }

        }

        if let Some(_super_class) = maybe_super_class {
            self.env.exit_scope();
        }

        self.env.assign(name, LoxValue::Class(Rc::new(RefCell::new(class))))?;


        Ok(())
    }
}