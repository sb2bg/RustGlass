use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::lang::interpreter::primitive::Primitive;

pub struct Scope<'a> {
    parent: Option<Rc<RefCell<Scope<'a>>>>,
    variables: HashMap<String, Rc<RefCell<Primitive>>>,
    name: &'a str,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<Rc<RefCell<Scope<'a>>>>, name: &'a str) -> Self {
        Scope {
            parent,
            variables: HashMap::new(),
            name,
        }
    }
}