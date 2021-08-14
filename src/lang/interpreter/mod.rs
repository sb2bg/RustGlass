/* for future reference
match (x, y) {
    (Int(i), Int(j)) => {…},
    (Str(s), Str(d)) => {…},
    _ => panic!("Type error!")
}
 */

use crate::lang::interpreter::scope::Scope;

pub mod primitive;
mod scope;

pub struct Interpreter {
    scope: Scope,
}

impl Interpreter {
    fn visit(&self, node: (/*Node*/)) -> (/*Primitive*/) {
        todo!()
    }

    fn visit_bin_op_node(&self, node: (/*BinOpNode*/)) -> (/*Primitive*/) {
        todo!()
    }
}