use proc_macro::TokenStream;
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, FnArg, GenericArgument, Ident, ItemFn, Lifetime, PathArguments,
    Type, parse_macro_input, parse_quote, parse_str, punctuated::Punctuated,
};

pub fn command_call(args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_ident = parse_macro_input!(args as Ident);
    let mut r#fn = parse_macro_input!(item as ItemFn);
    let fn_name = &r#fn.sig.ident;

    for arg in r#fn.sig.inputs.iter_mut() {
        if let FnArg::Typed(arg) = arg
            && let Type::Reference(r#ref) = &mut *arg.ty
            && let Type::Path(element) = &mut *r#ref.elem
            && let Some(segment) = element.path.segments.last_mut()
        {
            match segment.ident.to_string().as_str() {
                "CommandFeedback" => {
                    segment.arguments =
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: Punctuated::from_iter([GenericArgument::Lifetime(
                                Lifetime::new("'_", proc_macro2::Span::call_site()),
                            )]),
                            gt_token: Default::default(),
                        });
                    let path: syn::Path = parse_quote!(fluxer_rs::high_level::command_handler);
                    for seg in path.segments.into_iter().rev() {
                        element.path.segments.insert(0, seg);
                    }
                }
                "FluxerApiHandler" => {
                    let path: syn::Path = parse_quote!(fluxer_rs::api);
                    for seg in path.segments.into_iter().rev() {
                        element.path.segments.insert(0, seg);
                    }
                }
                _ => {}
            }
        }
    }

    r#fn.sig.output = syn::ReturnType::Type(
        Default::default(),
        Box::new(parse_str::<syn::Type>("Result<(), fluxer_rs::error::FluxerRsError>").unwrap()),
    );

    let expanded_macro = quote! {
        #r#fn

        pub struct #struct_ident { }

        impl fluxer_rs::high_level::command_handler::CommandTrait for #struct_ident {
            async fn execute<'a>(&self, api: &'a fluxer_rs::api::FluxerApiHandler, feedback: &'a fluxer_rs::high_level::command_handler::CommandFeedback<'a>) -> Result<(), fluxer_rs::error::FluxerRsError> {
                let _ = #fn_name(api, feedback).await;
                Ok(())
            }
        }
    };

    expanded_macro.into()
}
