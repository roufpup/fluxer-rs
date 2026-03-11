# About
A rust framework for interacting with fluxer instances

!!! Expect breaking changes !!!

For opening issues and feature requests please head over to the MIRROR repo over on https://github.com/roufpup/fluxer-rs

Feel free to join me and chat on Fluxer in the fluxer-rs server https://fluxer.gg/YpAOaODV

I will be doing my best to be implementing the API and gateway functionality as fast as i can so please bear with me.

# Example usage

```rs
// For more verbose messages like from the https client choose debug instead of info
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

let bot = FluxerBot::init(
    "<your bot token here>",
    "wss://gateway.fluxer.app?v=1&encoding=json&compress=none",
    "https://api.fluxer.app/v1",
)?;

bot.start(ColorbotDispatchHandler {}).await;
```
Each client using this crate must create their own dispatch handler by implementing the trait `DispatchHandlerTrait`. Once you do you can override the default functions of the dispatch handler that will be called for each dispatch event. The default functions are generated with a macro, and the name of each function follows this pattern: "handle_{}_dispatch" where {} is the name of the dispatch event in snake case. So the `MESSAGE_CREATE` dispatch event would have a function by the name of `handle_message_create_dispatch`

```rs
pub struct ColorbotDispatchHandler {}

impl DispatchHandlerTrait for ColorbotDispatchHandler {
    async fn handle_message_create_dispatch(
        &self,
        data: MessageData,
        api: &FluxerApiHandler,
    ) -> Result<(), FluxerRsError> {
        let mut cmd_handler = CommandHandler::init("!".to_string());

        register_commands!(cmd_handler,[
            {"addrole", AddRoleCommand},
            {"ping", PingCommand},
            {"edit", EditCommand},
            {"react", ReactCommand},
            {"removereact", RemoveReactCommand},
            {"removerole", RemoveRoleCommand},
            {"createrole", CreateRoleCommand},
            {"deleterole", DeleteRoleCommand},
        ]);

        cmd_handler.handle(&data, api).await
    }
}
```
This crate also will provide some high level implementations bots use in general, currently there only is a CommandHandler which will let you easily register custom commands for your bots as shown partially in the above example.

Here is an example of how to make a command for the CommandHandler

```rs
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(PingCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    send_reply(api, &data.channel_id, &data.id, "pong").await?;
    Ok(())
}
```
When creating a command, all you have to do is use the command macro and pass the command name which you later register to your dispatch handler. The signiture of the function must be the same as you see it in the example, an async function that takes two references, an api handler and a command feedback.

This crate also implements some basic functions that would be used a lot of the time which can be found under `fluxer_rs::api::common`, so you won't have to build an api call every time.

Of course you don't have to use this and are free to make your own better custom handler for commands :p

# Dispatch events
Not all op codes are implemented currently, and not all dispatch events have been registered yet. The reason for the latter is that i choose for the most part to implement data structure myself instead of generating it from a spec. So whenever an unknown dispatch event hits it would print the json that was attempted to be sent, then i implement the structure from it. Which leads to the next paragraph.

When developing a bot in debug mode, be wary as there is a certain panic that will occur when an unimplemented dispatch event occurs. It's there by design for my own ease of use ( So i have no choice but to implement the event if i don't want constant crashing ;3). While it's not optimal i do advise to use release mode for now while still a bunch of dispatch events are missing as it will only send an error log in the terminal for said dispatch event.

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
#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct EditMessage {
    pub channel_id: String,
    pub message_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
}

impl ApiCall for EditMessage {
    type ReturnType = MessageData;

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        let value = serde_json::to_string(self)?;

        Ok(req
            .body(value)
            .header("Authorization", format!("Bot {token}")))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Patch,
        )
    }

    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        let value = serde_json::from_str::<MessageData>(body)?;
        Ok(value)
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
        state.end()
    }
}
```
What you basically need to do is implement the ApiCall trait for the struct that will serve as your storage for parameters and potentially body data if required.

In this example taken right out of the crate you implement three functions for the struct, `get_req`, `get_info` and `get_data`, in the first one you construct the request with all the headers and body you need and return it. In the second you set some information for the api call like the path that will get called which will include also the path parameters taken out of the struct, and the http request method type. In the last function you return the data type that the api call would usually return. In a lot of cases that would be nothing then you just need to set `type ReturnType = ();` in your implementation. But in this case the api will return `MessageData`, so we deserialize the body and return it so it can be given to the client using the crate.

I've added a bit of custom serialization to make everything a bit easier when using a struct that shares both the serializable data and the path parameter data

If you'd like to implement an api call exactly like this in your own app you will have to add the `derive_builder` crate as a dependency as that is what provides the builder macros and generation.

After you have implemented the api call you simply call it like the common api functions do (see example below), the name of the builder is the same as the struct in this case it will be `EditMessageBuilder`

```rs
pub async fn edit_message(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    content: &str,
) -> Result<MessageData, FluxerRsError> {
    let call = EditMessageBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .content(content)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}
```