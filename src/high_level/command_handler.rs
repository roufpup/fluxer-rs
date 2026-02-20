use std::{collections::HashMap, pin::Pin};

use log::info;

use crate::gateway::data_structure::message::MessageEventData;

pub type Command = Box<dyn BoxedCommandTrait + Send + Sync>;

pub trait CommandTrait: Send + Sync + 'static {
    fn execute(&self) -> impl Future<Output = ()> + Send;
}

pub trait BoxedCommandTrait {
    fn execute(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

impl<C: CommandTrait> BoxedCommandTrait for C {
    fn execute(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(CommandTrait::execute(self))
    }
}

pub struct CommandHandler {
    prefix: String,
    map: HashMap<String, Command>,
}

impl CommandHandler {
    pub fn init(prefix: String) -> Self {
        CommandHandler {
            prefix,
            map: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, command_name: String, handler: impl CommandTrait + 'static) {
        self.map.insert(command_name, Box::new(handler));
    }

    pub async fn handle(&self, data: &MessageEventData) {
        let msg_no_pfx = Self::remove_pfx(&self.prefix, &data.content).await;
        if let Some((command, _)) = msg_no_pfx {
            match self.map.get(&command) {
                Some(handler) => handler.execute().await,
                None => info!("Unknown command"),
            }
        }
    }

    pub async fn remove_pfx(prefix: &str, content: &str) -> Option<(String, String)> {
        #[allow(clippy::manual_map)]
        match content.split_once(prefix) {
            Some(content) => match content.1.split_once(" ") {
                Some((command, body)) => Some((command.to_string(), body.to_string())),
                None => Some((content.1.to_string(), "".to_string())),
            },
            None => None,
        }
    }
}
