//! Runtime environment


use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::LoxValue;
use crate::error::RloxError;
use crate::ast::token::Token;
use crate::ast::token::TokenType;

#[derive(Debug)]
pub struct EnvItem {
    pub table: HashMap<String, LoxValue>,
    pub parent: Option<Rc<RefCell<EnvItem>>>,
}

impl EnvItem {
    pub fn from_parent(parent: Rc<RefCell<EnvItem>>) -> Self {
        EnvItem {
            table: HashMap::new(),
            parent: Some(parent),
        }
    }
}

impl ToString for EnvItem {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("EnvItem:\n");
        s.push_str("\tkeys: ");
        for key in self.table.keys() {
            s.push_str(&format!("{} ", key));
        }
        s.push_str("\n");
        s.push_str("\tparent: ");
        if let Some(parent) = &self.parent {
            s.push_str(parent.borrow().to_string().as_str());
        } else {
            s.push_str("None");
        }
        s
    }
}

#[derive(Debug)]
pub struct Environment {
    pub call_stack: usize,
    pub values: Rc<RefCell<EnvItem>>,
    pub global: Rc<RefCell<EnvItem>>,
}

static MAX_CALL_STACK: usize = 255;

impl Environment {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(
            EnvItem {
                table: HashMap::new(),
                parent: None,
            }
        ));
        Environment {
            call_stack: 0,
            global: Rc::clone(&global),
            values: Rc::clone(&global),
        }
    }

    pub fn from(call_stack: usize, global: Rc<RefCell<EnvItem>>, closure: Rc<RefCell<EnvItem>>) -> Result<Self, RloxError> {
        if call_stack > MAX_CALL_STACK {
            Err(RloxError::RuntimeError("Stack overflow.".to_string()))
        } else {
            Ok(Environment {
            call_stack: call_stack,
            global: Rc::clone(&global),
            values: Rc::clone(&closure),
        })
        }
    }

    /// Enter a new scope, which will push a new table onto the stack. 
    pub fn enter_scope(&mut self) {
        // let curr_stack = std::mem::take(&mut self.values);
        // self.values = StackItem::Table(HashMap::new(), Box::new(curr_stack));
        self.values = Rc::new(RefCell::new(
            EnvItem {
                table: HashMap::new(),
                parent: Some(Rc::clone(&self.values)),
            }
        ));
    }

    /// Exit the current scope, which will pop the top table from the stack. <br>
    pub fn exit_scope(&mut self) {
        let parent = self.values.borrow_mut().parent.clone();
        if let Some(parent) = parent {
            // NOTE: we must clear the current table to avoid memory leaks
            // 如果在这里不清空当前的 table，table 中的 function closure 可能会引用这个 EnvItem，循环引用导致内存泄漏
            // self.values.borrow_mut().table.clear();
            self.values = parent;
        } else {
            panic!("No parent scope to exit to");
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(format!("Invalid token type '{}'.", name.lexeme)));
        }
        let name = &name.lexeme;
        let mut current = Rc::clone(&self.values);
        loop {
            if let Some(v) = current.borrow_mut().table.get_mut(name) {
                *v = value;
                return Ok(());
            }
            {
                let mut _next: Option<Rc<RefCell<EnvItem>>> = None;
                if let Some(parent) = &current.borrow_mut().parent {
                    _next = Some(Rc::clone(parent));
                } else {
                    break;
                }
                current = _next.unwrap();
            }
        }
        Err(RloxError::RuntimeError(format!("Undefined variable '{}'.", name)))
    }

    pub fn assign_by_depth(&mut self, name: &Token, value: LoxValue, depth: usize) -> Result<(), RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(format!("Invalid token type '{}'.", name.lexeme)));
        }
        let name = &name.lexeme;
        let current = self.ancestor(depth);
        if let Some(v) = current.borrow_mut().table.get_mut(name) {
            *v = value;
            return Ok(());
        }
        Err(RloxError::RuntimeError(format!("Undefined variable '{}'.", name)))
    }

    pub fn define_globally(&mut self, name: &str, value: LoxValue) {
        self.global.borrow_mut().table.insert(name.to_string(), value);
    }

    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.values.borrow_mut().table.insert(name.to_string(), value);
    }

    fn get_helper(values: &Rc<RefCell<EnvItem>>, name: &str) -> Option<LoxValue> {
        let values = values.borrow();
        if let Some(value) = values.table.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &values.parent {
            Self::get_helper(parent, name)
        } else {
            None
        }
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier && name.t_type != TokenType::This && name.t_type != TokenType::Super {
            return Err(RloxError::RuntimeError(format!("Invalid token type '{}'.", name.lexeme)));
        }
        match Self::get_helper(&self.values, &name.lexeme) {
            Some(value) => Ok(value),
            None => Err(RloxError::RuntimeError(format!("Undefined variable '{}'.", name.lexeme))),
        }
    }

    pub fn get_by_depth(&self, name: &Token, depth: usize) -> Result<LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier && name.t_type != TokenType::This && name.t_type != TokenType::Super {
            return Err(RloxError::RuntimeError(format!("Invalid token type '{}'.", name.lexeme)));
        }
        // go to depth
        let current = self.ancestor(depth);
        let res = match current.borrow().table.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RloxError::RuntimeError(format!("Undefined variable '{}'.", name.lexeme))),
        };
        return res;
    }

    fn ancestor(&self, depth: usize) -> Rc<RefCell<EnvItem>> {
        let mut current = Rc::clone(&self.values);
        for _ in 0..depth {
            let parent = {
                let current_borrow = current.borrow();
                current_borrow.parent.as_ref().map(Rc::clone)
            };
            if let Some(parent) = parent {
                current = parent;
            } else {
                panic!("No parent scope to exit to");
            }
        }
        current
    }
    
    
}