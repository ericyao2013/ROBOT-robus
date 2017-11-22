use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use {Message, Module};
use msg::TargetMode;

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
        match msg.header.target_mode {
            TargetMode::Broadcast => self.modules.clone(),
            TargetMode::Id => {
                let module = self.modules.iter().find(|mod_ref| {
                    mod_ref.borrow().id == msg.header.target
                });
                match module {
                    Some(module) => vec![module.clone()],
                    None => vec![],
                }
            }
            _ => vec![],
        }
    }
}
