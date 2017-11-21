use std::rc::Rc;
use std::cell::RefCell;

use {Message, Module, ModuleType};

use msg::MAX_MESSAGE_SIZE;
use registry::Registry;
use recv_buf::RecvBuf;

pub struct Core<'a> {
    registry: Registry<'a>,
    recv_buf: RecvBuf,
}

impl<'a> Core<'a> {
    pub fn new() -> Core<'a> {
        Core {
            registry: Registry::new(),
            recv_buf: RecvBuf::with_capacity(MAX_MESSAGE_SIZE),
        }
    }
    pub fn create_module(
        &mut self,
        alias: &'a str,
        mod_type: ModuleType,
        cb: &'a Fn(&Message),
    ) -> Rc<RefCell<Module<'a>>> {

        let module = Module::new(alias, mod_type, cb);
        let mod_ref = Rc::new(RefCell::new(module));

        self.registry.add(mod_ref.clone());

        mod_ref
    }
    pub fn receive(&mut self, byte: u8) {
        self.recv_buf.push(byte);

        if let Some(msg) = self.recv_buf.get_message() {
            let matches = self.registry.find_targeted_modules(&msg);

            for mod_ref in matches.iter() {
                let module = mod_ref.borrow();
                (module.callback)(&msg);
            }
        }
    }
    pub fn send(&self, from: &Rc<RefCell<Module>>, mut msg: &mut Message) {
        let module = from.borrow();
        module.send(&mut msg);
    }
}
