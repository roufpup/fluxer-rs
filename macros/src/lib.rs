mod command;
mod dispatch;
mod register_command;

use proc_macro::TokenStream;

use crate::{
    command::command_call, dispatch::dispatch_call, register_command::register_command_call,
};

#[proc_macro]
pub fn dispatch(stream: TokenStream) -> TokenStream {
    dispatch_call(stream)
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
    command_call(args, item)
}

#[proc_macro]
pub fn register_commands(stream: TokenStream) -> TokenStream {
    register_command_call(stream)
}
