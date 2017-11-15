mod mod_type;
pub use self::mod_type::ModuleType;

use Message;

const MAX_ALIAS_SIZE: usize = 15;
const DEFAULT_ID: u16 = 0;

pub struct Module<'a> {
    pub alias: &'a str,
    pub mod_type: ModuleType,
    pub id: u16,
}

impl<'a> Module<'a> {
    pub fn new(alias: &str, mod_type: ModuleType) -> Module {
        Module {
            alias,
            id: DEFAULT_ID,
            mod_type,
        }
    }
    pub fn send(&self, msg: &Message) {}
    pub fn read(&self) -> Option<Message> {
        None
    }

    pub fn set_cb<F>(&self, cb: F)
    where
        F: Fn(&Message),
    {
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    use super::super::msg::tests::{rand_id, rand_msg};

    #[test]
    fn module_setup() {
        let alias = rand_alias();
        let mod_type = rand_type();

        let module = Module::new(&alias, mod_type);

        assert_eq!(module.alias, alias);
        assert_eq!(module.id, DEFAULT_ID);
        assert_eq!(module.mod_type, mod_type);
    }

    #[test]
    #[should_panic]
    fn bad_alias() {
        let mut rng = rand::thread_rng();

        let offset = rng.gen_range(1, 100);
        let s = rng.gen_ascii_chars()
            .take(MAX_ALIAS_SIZE + offset)
            .collect::<String>();

        Module::new(&s, rand_type());
    }

    #[test]
    fn fill_source_on_send() {
        let alias = rand_alias();

        let mut module = Module::new(&alias, rand_type());
        module.id = rand_id();

        let msg = rand_msg();
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
