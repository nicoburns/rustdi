extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
use crate::proc_macro::{TokenStream};
use crate::proc_macro2::{Span};
use crate::quote::{quote, quote_spanned};

mod ioc;
use self::ioc::ServiceContainer;

use syn::{Item, ItemFn, FnArg, ArgCaptured, Type, TypePath, TypeReference, Path, Ident};

enum ResolveType {
    ImmutableBorrow,
    MutableBorrow,
}
use self::ResolveType::*;

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, input: TokenStream) -> TokenStream {

    // Parse input as a function (or panic)
    println!("input: \"{}\"", input.clone().to_string());
    let func : ItemFn = syn::parse(input.clone()).expect("The inject macro is only supported on functions");

    let arg_types_and_mutabilities = func.clone().decl.inputs.into_iter()
        .map(|arg| {
            if let FnArg::Captured(ArgCaptured{ ty: arg_type, .. }) = arg {
                if let Type::Reference(TypeReference{ mutability, elem, .. }) = arg_type.clone() {
                    if let Type::Path(TypePath{ qself: None, path: arg_path }) = *elem {

                        // Print debug info
                        let arg_mutability_str = match &mutability { Some(_) => "&mut ", None => "&"};
                        let arg_path_str = arg_path.clone().segments.into_iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::");
                        println!("type: {}{}", arg_mutability_str, arg_path_str);

                        let arg_mutability = match &mutability { Some(_) => MutableBorrow, None => ImmutableBorrow };
                        (arg_type, arg_path, arg_mutability)

                    } else { panic!("The inject macro only supports simple type arguments");}
                } else { panic!("The inject macro only supports simple type arguments"); }
            } else { panic!("The inject macro only supports simple type arguments"); }
        });

    // Span types
    let ident = func.ident.clone();
    let container_type = quote_spanned!{Span::call_site() => &crate::ioc::ServiceContainer};
    let original_func_ident = Ident::new(format!("{}_orig", ident).as_str(), ident.span());;
    let mut original_func = func.clone();
    original_func.ident = original_func_ident.clone();

    let args = arg_types_and_mutabilities.map(|(arg_type, arg_path, arg_mutability)| {
        match arg_mutability {
            ImmutableBorrow => quote_spanned!{Span::call_site() => &*resolver.resolve_read::<#arg_path>().unwrap()},
            MutableBorrow   => quote_spanned!{Span::call_site() => &mut*resolver.resolve_write::<#arg_path>().unwrap()},
        }
    });


    return quote!{
        
        fn #ident(resolver: #container_type) {
            #original_func
            #original_func_ident(#(#args),*);
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

