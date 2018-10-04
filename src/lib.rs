extern crate proc_macro;
use crate::proc_macro::TokenStream;

use syn::{Item, FnArg, ArgCaptured, Type, TypePath, Path};

#[proc_macro_attribute]
pub fn show_streams(_attr: TokenStream, input: TokenStream) -> TokenStream {
    
    //println!("attr: \"{}\"", attr.to_string());
    //println!("input: \"{}\"", input.to_string());
    // println!("{}", input..c.count());

    // for item in input.clone() {
    //     println!("{}", item.to_string());
    // }

    let item : Item = syn::parse(input.clone()).expect("failed to parse input");

    let func = match item {
        Item::Fn(func) => func,
        _ => panic!("The macro is only supported on functions"),
    };

    for arg in func.decl.inputs {

        //if let FnArg::Captured(ArgCaptured{ty: Type::Verbatim, ..})
        //println!("{:?}\n", arg);

        //if let Captured(ArgCaptured { pat: Ident(PatIdent { by_ref: None, mutability: None, ident: Ident { ident: "_s3", span: #0 bytes(228..231) }, subpat: None }), .. })
        if let FnArg::Captured(ArgCaptured { ty: Type::Path(TypePath { qself: None, path: Path { segments, .. } }), .. }) = arg {
            println!("type: {}", segments.into_iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::"));
            //[PathSegment { ident: ident, .. }]
        }

    }

    input
}

