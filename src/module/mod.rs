mod mod_type;
pub use self::mod_type::ModuleType;

use Message;

const MAX_ALIAS_SIZE: usize = 15;
const DEFAULT_ID: u16 = 0;

pub struct Module<'a> {
    pub alias: &'a str,
    pub mod_type: ModuleType,
    pub id: u16,
    callback: Box<Fn(&Message) + 'a>,
}

impl<'a> Module<'a> {
    pub fn new(alias: &str, mod_type: ModuleType, cb: Box<Fn(&Message)>) -> Module {
        if alias.len() > MAX_ALIAS_SIZE {
            panic!("alias size({}) out of range.", alias.len());
        }
        Module {
            alias,
            id: DEFAULT_ID,
            mod_type,
            callback: cb,
        }
    }
    pub fn send(&self, msg: &mut Message) {
        msg.header.source = self.id;
        // TODO : compute CRC
        // manage tx_lock
        // hal::send(&msg.to_bytes(), msg.header.data_size)
        println!("{:?}", msg);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    use super::super::msg::tests::{rand_id, rand_msg};
    fn callback(msg: &Message) {}

    #[test]
    fn module_setup() {
        let alias = rand_alias();
        let mod_type = rand_type();

        let module = Module::new(&alias, mod_type, Box::new(callback));

        assert_eq!(module.alias, alias);
        assert_eq!(module.id, DEFAULT_ID);
        assert_eq!(module.mod_type, mod_type);
    }

    #[test]
    #[should_panic]
    fn bad_alias() {
        let mut rng = rand::thread_rng();

        let bad_size = rng.gen_range(MAX_ALIAS_SIZE, MAX_ALIAS_SIZE + 100);
        let s = rng.gen_ascii_chars().take(bad_size).collect::<String>();

        Module::new(&s, rand_type(), Box::new(callback));
    }

    #[test]
    fn fill_source_on_send() {
        let alias = rand_alias();

        let mut module = Module::new(&alias, rand_type(), Box::new(callback));
        module.id = rand_id();

        let mut msg = rand_msg();
        module.send(&mut msg);
        assert_eq!(msg.header.source, module.id);
    }
    fn rand_alias<'a>() -> String {
        let mut rng = thread_rng();

        let length = rng.gen_range(1, MAX_ALIAS_SIZE);
        rng.gen_ascii_chars().take(length).collect()
    }
    fn rand_type() -> ModuleType {
        ModuleType::Button
    }
}
