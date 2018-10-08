extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use crate::proc_macro::{TokenStream};
use crate::proc_macro2::{Span};

use syn::{ItemFn, FnArg, ArgCaptured, Type, ReturnType, TypePath, TypeReference, Ident};
use quote::ToTokens;

enum ResolveType {
    ImmutableBorrow,
    MutableBorrow,
    OwnedValue,
}

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, input: TokenStream) -> TokenStream {

    // Parse input as a function (or panic)
    let func : ItemFn = syn::parse(input.clone()).expect("The inject macro is only supported on functions");

    // Extract argument info from function
    let arg_types_and_mutabilities = func.clone().decl.inputs.into_iter()
        .map(|arg| {
            match arg {
                FnArg::Captured(ArgCaptured{ ty: Type::Reference(TypeReference{ mutability, elem, .. }), .. }) => {
                    if let Type::Path(TypePath{ qself: None, path: arg_path }) = *elem {

                        let arg_mutability = match &mutability {
                            Some(_) => ResolveType::MutableBorrow,
                            None    => ResolveType::ImmutableBorrow
                        };
                        (arg_path, arg_mutability)

                    } else { panic!("The inject macro only supports simple type arguments"); }
                },
                FnArg::Captured(ArgCaptured{ ty: Type::Path(TypePath{ qself: None, path: arg_path }), .. }) => {
                    (arg_path, ResolveType::OwnedValue)
                },
                _ => panic!("The inject macro only supports simple type arguments"),
            }
        });

    
    // Generate parts of the output function
    let ident = func.ident.clone();
    let return_type = match func.decl.output.clone() {
        ReturnType::Default => Box::new(quote!{()}) as Box<ToTokens>,
        ReturnType::Type(_, ty) => ty as Box<ToTokens>,
    };
    //let container_type = quote_spanned!{Span::call_site() => &::rustdi::ServiceContainer};
    let resolver_trait = quote_spanned!{Span::call_site() => ::rustdi::Resolver};
    let original_func_ident = Ident::new(format!("{}_orig", ident).as_str(), ident.span());;
    let mut original_func = func.clone();
    original_func.ident = original_func_ident.clone();

    // Generate code to resolve injected arguments from container with requested mutability
    let args = arg_types_and_mutabilities.map(|(arg_path, arg_mutability)| {
        match arg_mutability {
            ResolveType::ImmutableBorrow => quote_spanned!{Span::call_site() => &*resolver.resolve_immutable_ref::<#arg_path>()?},
            ResolveType::MutableBorrow   => quote_spanned!{Span::call_site() => &mut*resolver.resolve_mutable_ref::<#arg_path>()?},
            ResolveType::OwnedValue      => quote_spanned!{Span::call_site() => resolver.resolve_owned_value::<#arg_path>()?},
        }
    });

    // Write out new wrapped function
    return quote!{
        
        fn #ident<R: #resolver_trait>(resolver: &R) -> Result<#return_type, R::Error> {
            #original_func
            let ret = #original_func_ident(#(#args),*);
            return Ok(ret);
        }

    }.into();
}

// #[proc_macro_attribute]
// pub fn show_streams(_attr: TokenStream, input: TokenStream) -> TokenStream {
    
//     //println!("attr: \"{}\"", attr.to_string());
//     //println!("input: \"{}\"", input.to_string());
//     // println!("{}", input..c.count());

//     // for item in input.clone() {
//     //     println!("{}", item.to_string());
//     // }

//     let item : Item = syn::parse(input.clone()).expect("failed to parse input");

//     let func = match item {
//         Item::Fn(func) => func,
//         _ => panic!("The macro is only supported on functions"),
//     };

//     for arg in func.decl.inputs {

//         //if let FnArg::Captured(ArgCaptured{ty: Type::Verbatim, ..})
//         //println!("{:?}\n", arg);

//         //if let Captured(ArgCaptured { pat: Ident(PatIdent { by_ref: None, mutability: None, ident: Ident { ident: "_s3", span: #0 bytes(228..231) }, subpat: None }), .. })
//         if let FnArg::Captured(ArgCaptured { ty: Type::Path(TypePath { qself: None, path: Path { segments, .. } }), .. }) = arg {
//             println!("type: {}", segments.into_iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::"));
//             //[PathSegment { ident: ident, .. }]
//         }

//     }

//     input
// }

