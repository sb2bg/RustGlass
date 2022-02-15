/* for future reference
match (x, y) {
    (Int(i), Int(j)) => {…},
    (Str(s), Str(d)) => {…},
    _ => panic!("Type error!")
}
 */

use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::interpreter::scope::Scope;

pub mod primitive;
mod scope;

pub struct Interpreter<'a> {
    scope: Scope<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parent: Option<Rc<RefCell<Scope<'a>>>>, scope_name: &'a str) -> Self {
        Interpreter {
            scope: Scope::new(None, scope_name),
        }
    }

    fn visit(&self, node: (/*Node*/)) -> (/*Primitive*/) {
        todo!()
    }

    fn visit_bin_op_node(&self, node: (/*BinOpNode*/)) -> (/*Primitive*/) {
        todo!()
    }
}