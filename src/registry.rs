use std::rc::Rc;
use std::cell::RefCell;

use {Message, Module};

pub struct Registry<'a> {
    modules: Vec<Rc<RefCell<Module<'a>>>>,
}

impl<'a> Registry<'a> {
    pub fn new() -> Registry<'a> {
        Registry { modules: vec![] }
    }
    pub fn add(&mut self, mod_ref: Rc<RefCell<Module<'a>>>) {
        self.modules.push(mod_ref);
    }
    pub fn find_targeted_modules(&self, msg: &Message) -> Vec<Rc<RefCell<Module<'a>>>> {
        // TODO: find the modules targeted by the message
        vec![]
    }
}
