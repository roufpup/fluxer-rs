use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token, Type, braced, bracketed, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

pub struct MacroData {
    handler: Ident,
    commands: Vec<CommandsData>,
}

impl Parse for MacroData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let handler = input.parse::<Ident>().unwrap();
        input.parse::<Token![,]>().unwrap();
        let bracketed_data;
        bracketed!(bracketed_data in input);

        let commands = Punctuated::<CommandsData, Token![,]>::parse_terminated(&bracketed_data)
            .unwrap()
            .into_iter()
            .collect::<Vec<CommandsData>>();
        Ok(MacroData { handler, commands })
    }
}

pub struct CommandsData {
    name: LitStr,
    expr: Type,
}

impl Parse for CommandsData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let braced_data;
        braced!(braced_data in input);

        let name = braced_data.parse::<LitStr>().unwrap();
        braced_data.parse::<Token![,]>().unwrap();
        let expr = braced_data.parse::<Type>().unwrap();
        Ok(CommandsData { name, expr })
    }
}

pub fn register_command_call(stream: TokenStream) -> TokenStream {
    let data = parse_macro_input!(stream as MacroData);
    let elems = (0..data.commands.len()).map(|i| {
        let commands_data = data.commands.get(i).unwrap();
        let handler_ident = &data.handler;
        let lit_name = &commands_data.name;
        let expr = &commands_data.expr;

        quote! {
            #handler_ident.register_command(
                #lit_name.to_string(),
                #expr {},
            );
        }
    });

    let expanded_macro = quote! {
        #(#elems)*
    };

    expanded_macro.into()
}
