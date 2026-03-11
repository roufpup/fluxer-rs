use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitStr, PathArguments, Token, Type, bracketed, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

struct MacroData {
    data_list: Vec<DispatchData>,
}

impl Parse for MacroData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let data_list: Vec<DispatchData> =
            Punctuated::<DispatchData, Token![,]>::parse_terminated(input)
                .unwrap()
                .into_iter()
                .collect();

        Ok(MacroData { data_list })
    }
}

struct DispatchData {
    event_name: LitStr,
    data_type: Type,
}

impl Parse for DispatchData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let data;
        bracketed!(data in input);

        let event_name = data.parse::<LitStr>().unwrap();
        data.parse::<Token![,]>().unwrap();
        let data_type = data.parse::<Type>().unwrap();

        Ok(DispatchData {
            event_name,
            data_type,
        })
    }
}

pub fn dispatch_call(stream: TokenStream) -> TokenStream {
    let dispatch: MacroData = parse_macro_input!(stream as MacroData);
    let count = dispatch.data_list.len();
    let trait_fns = (0..count).map(|i| {
        let dispatch_data = dispatch.data_list.get(i).unwrap();

        let event_name = dispatch_data.event_name.value();
        let r#type = &dispatch_data.data_type;

        if event_name == "None" {
            return quote! {};
        }

        let fn_name = format_ident!("handle_{}_dispatch", event_name.to_snake_case());

        let info_message = format!("-> [DISPATCH::{}]", event_name);

        quote! {
            fn #fn_name(
                &self,
                _data: #r#type,
                _api: &crate::api::FluxerApiHandler
            ) -> impl Future<Output = Result<(), FluxerRsError>> + Send {
                async move {
                    info!(#info_message);
                    Ok(())
                }
            }
        }
    });

    let enum_elems = (0..count).map(|i| {
        let dispatch_data = dispatch.data_list.get(i).unwrap();
        let event_name = dispatch_data.event_name.value();
        let r#type = &dispatch_data.data_type;

        if event_name == "None" {
            let name_ident = format_ident!("None");
            return quote! {
                #name_ident
            };
        }

        let name_ident = format_ident!("{}", event_name.to_upper_camel_case());

        quote! {
            #name_ident(#r#type)
        }
    });

    let match_elems = (0..count).map(|i| {
        let dispatch_data = dispatch.data_list.get(i).unwrap();
        let event_name = dispatch_data.event_name.value();
        let match_ident = format_ident!("{}", event_name.to_upper_camel_case());

        if event_name == "None" {
            return quote! {
                DispatchEvent::#match_ident => Ok(()),
            };
        }

        let handler_ident = format_ident!("handle_{}_dispatch", event_name.to_snake_case());

        quote! {
            DispatchEvent::#match_ident(data) => handler.#handler_ident(data,api).await,
        }
    });

    let deserialize_elems = (0..count).map(|i| {
        let dispatch_data = dispatch.data_list.get(i).unwrap();
        let event_name = &dispatch_data.event_name;
        let match_ident = format_ident!("{}", event_name.value().to_upper_camel_case());

        let mut r#type = dispatch_data.data_type.clone();
        let Type::Path(path) = &mut r#type else {
            return quote! {};
        };

        if path.path.segments.last().unwrap().ident == "Option" {
            return quote! {};
        }

        if path.path.segments.last().unwrap().ident == "Vec" {
            path.path.segments.iter_mut().for_each(|a| {
                if let PathArguments::AngleBracketed(args) = &mut a.arguments {
                    args.colon2_token = Some(Default::default())
                }
            });
        };

        quote! {
            #event_name => DispatchEvent::#match_ident(
                #r#type::deserialize(&value["d"]).map_err(de::Error::custom)?,
            ),
        }
    });

    let expanded_macro = quote! {
        use crate::{
            serde::types::{
                common::*,
                user::*,
                guild::*,
                message::*,
            },
            api::FluxerApiHandler,
            error::FluxerRsError
        };
        use async_trait::async_trait;
        use anyhow::Result;
        #[allow(unused_imports)]
        use log::{info,error};
        use serde::{Deserialize, de};


        pub enum DispatchEvent {
            #(#enum_elems,)*
        }

        pub trait DispatchHandlerTrait {
            #(#trait_fns)*
        }

        #[derive(Default)]
        pub struct DispatchHandler;
        #[async_trait]
        impl DispatchHandlerTrait for DispatchHandler {}

        pub async fn handle_dispatch_events<T: DispatchHandlerTrait + Send + Sync + 'static>(
            dispatch_event: Box<DispatchEvent>,
            handler: &T,
            api: &FluxerApiHandler
        ) -> Result<(), FluxerRsError> {
            match *dispatch_event {
                #(#match_elems)*
            }
        }

        pub fn dispatch_deserialize(value: &serde_json::value::Value) -> Result<DispatchEvent, serde_json::Error>{
            Ok(
                match value["t"].as_str().ok_or_else(||de::Error::custom("Failed to find field t"))? {
                #(#deserialize_elems)*

                #[cfg(not(debug_assertions))]
                _ => {
                    error!("Unimplemented dispatch event with data: {}", value);
                    DispatchEvent::None
                }

                #[cfg(debug_assertions)]
                _ => panic!("Unimplemented dispatch event with data: {}", value),
            }
            )
        }
    };
    expanded_macro.into()
}
