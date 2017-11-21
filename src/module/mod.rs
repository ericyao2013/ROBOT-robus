mod mod_type;
pub use self::mod_type::ModuleType;

use Message;

const MAX_ALIAS_SIZE: usize = 15;
const DEFAULT_ID: u16 = 0;

/// Robus Module struct used for representing actuators and sensors
///
/// ## Examples
/// ```
/// use robus::{Command, Message};
///
/// let (tx, _) = robus::message_queue();
///
/// let cb = move |msg: &Message| {
///     let answer = match msg.header.command {
///         Command::Identify => Some(Message::id(
///             1,
///             Command::Introduction,
///             &"hello".as_bytes().to_owned(),
///         )),
///         Command::GetState => Some(Message::id(
///             1,
///             Command::PublishState,
///             &vec![42],
///         )),
///         _ => None,
///     };
///     if let Some(answer) = answer {
///         tx.send(answer);
///     }
/// };
///
/// let module = robus::Module::new(
///        "fire_button",
///        robus::ModuleType::Button,
///        &cb,
///    );
/// ```
pub struct Module<'a> {
    /// Each module have a name allowing to users to manage them easily.
    pub alias: &'a str,
    /// A `ModuleType` defining the hardware category of the module.
    pub mod_type: ModuleType,
    /// The unique id of the module needed to send/receive specific messages.
    pub id: u16,
    /// This callback is called on message reception for this module.
    pub callback: &'a Fn(&Message),
}

impl<'a> Module<'a> {
    /// Creates a new a Module.
    ///
    /// # Arguments
    ///
    /// * `alias` - A `&str` containing the module name (max length is 15).
    /// * `mod_type` - A `ModuleType` struct designating the hardware category of the module.
    /// * `cb` - A `Fn(&Message)` containing the function to call at message reception.
    pub fn new(alias: &'a str, mod_type: ModuleType, callback: &'a Fn(&Message)) -> Module<'a> {
        if alias.len() > MAX_ALIAS_SIZE {
            panic!("alias size({}) out of range.", alias.len());
        }
        let module = Module {
            alias,
            id: DEFAULT_ID,
            mod_type,
            callback,
        };
        module
    }

    /// Sends a message from this module.
    ///
    /// # Arguments
    ///
    /// * `msg` - The `Message` to send.
    pub fn send(&self, msg: &mut Message) {
        msg.header.source = self.id;
        // TODO : compute CRC
        // manage tx_lock
        // hal::send(&msg.to_bytes(), msg.header.data_size)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    use super::super::msg::tests::{rand_id, rand_msg};
    fn callback(_msg: &Message) {}

    #[test]
    fn module_setup() {
        let alias = rand_alias();
        let mod_type = rand_type();

        let module = Module::new(&alias, mod_type, &callback);

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

        Module::new(&s, rand_type(), &callback);
    }

    #[test]
    fn fill_source_on_send() {
        let alias = rand_alias();

        let mut module = Module::new(&alias, rand_type(), &callback);
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
