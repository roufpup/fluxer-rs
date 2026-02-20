# About
A rust implementation of the fluxer api to be used on fluxer.app and other self hosted instances

For opening issues and feature requests please head over to the MIRROR repo over on https://github.com/roufpup/fluxer-rs

Feel free to join me and chat on Fluxer in the fluxer-rs server https://fluxer.gg/YpAOaODV

I will be doing my best to be implementing the API and gateway functionality as fast as i can so please bear with me.

# Example usage

```rs

 let bot: FluxerBot = FluxerBot::init(
     "<Your bot token here>".to_string(),
     "wss://gateway.fluxer.app?v=1&encoding=json&compress=none".to_string(),
     "https://api.fluxer.app/v1".to_string(),
 )
 .await;

 let bot_arc = Arc::new(bot);
 bot_arc
     .start(ColorbotDispatchHandler {
         bot: bot_arc.clone(),
     })
     .await;
```
We pass an Arc of the bot to the dispatch handler implementation so we can use the api of our above created bot inside different dispatch callbacks as shown below

```rs

pub struct ColorbotDispatchHandler {
    pub bot: Arc<FluxerBot>,
}

impl DispatchHandlerTrait for ColorbotDispatchHandler {
    async fn handle_message_create_dispatch(&self, data: MessageEventData) {
        let mut cmd_handler = CommandHandler::init("!".to_string());

        cmd_handler.register_command(
            "ping".to_string(),
            PingCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                id: data.id.clone(),
            },
        );

        cmd_handler.handle(&data).await;
    }
```
This is also another thing to touch on how to reply to different dispatch events. All you need to do is implement the DispatchHandlerTrait for your struct and implement the functions that you want to override instead of letting the crate use the predefined default ones.

This crate also will provide some high level implementations bots use in general, currently there only is a CommandHandler which will let you easily register custom commands for your bots.

Here is an example of how to make a command for the CommandHandler

```rs
pub struct PingCommand {
    pub bot: Arc<FluxerBot>,
    pub channel_id: String,
    pub id: String,
}

impl CommandTrait for PingCommand {
    async fn execute(&self) {
        let _ = self.bot.api.execute_call(
            SendMessageBuilder::default()
                .channel_id(self.channel_id.clone())
                .content("pong".to_string())
                .message_reference(
                    MessageReferenceBuilder::default()
                        .message_id(self.id.clone())
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );
    }
}
```
When creating a command we pass all the required data into the command struct and just implement the CommandTrait for it. In this example the execute function will simply send a message back to the user by replying to their original message with `pong`

Of course you don't have to use this and are free to make your own better custom handler for commands :p

# Dispatch events
Not all dispatch events are implemented currently as i am limited on time for how much i can do in such a short period of time, but i will be implementing them constantly as much as possible as fast as possible.

When developing a bot in debug mode, be wary as there is a certain panic function that will occur when an unimplemented dispatch event occurs. It's there by design for my own ease of use ( So i have no choice but to implement the event if i don't want constant crashing ;3). While it's not optimal i do advise to use release mode for now while still a bunch of dispatch events are missing.

# API
As far as API calls go there are very few implemented at the moment but are quite useful ones like:
- Message sending ( Including embeds )
- Message editing
- Message fetching
- Adding the bot's own reaction to a message
- Removing all reactions from a message
- Creating a role
- Deleting a role
- Giving a role to a user
- Removing a role from a user

Whenever an api call is missing but you really want to use it be not afraid you can also implement an API call on your own in the meanwhile until it gets added. Here is an example:

```rs
#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct EditMessage {
    // Path params
    pub channel_id: String,
    pub message_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
    #[builder(default)]
    pub message_reference: Option<MessageReference>,
}


impl ApiCall for EditMessage {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        let body = serde_json::to_string(self).unwrap();
        info!("BODY CHECK {body}");
        req.with_body(body)
            .with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Patch,
        )
    }
}

impl Serialize for EditMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("content", &self.content)?;
        if let Some(embed) = self.embeds.clone() {
            state.serialize_entry("embeds", &embed)?;
        };
        if let Some(message_reference) = self.message_reference.clone() {
            state.serialize_entry("message_reference", &message_reference)?;
        }
        state.end()
    }
}
```
What you basically need to do is implement the ApiCall trait for the struct that will serve as your storage for parameters and potentially body data if required.

In this example taken right out of the crate you implement two functions for the struct, `get_req` and `get_info`, in the first one you construct the request with all the headers and body you need and return it. While in the second you set some information for the api call like the path that will get called which will include also the path parameters taken out of the struct as well as the http request method type.

I've added a bit of custom serialization to make everything a bit easier when using a struct that shares both the serializable data and the path parameter data

If you'd like to implement an api call exactly like this in your own app you will have to add the `derive_builder` crate as a dependency as that is what provides the builder macros and generation.

After you have implemented the api call you simply call it like in the example above for the PingCommand, the name of the builder is the same as the struct in this case it will be `EditMessageBuilder`