extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{ parse_macro_input};
use syn::{ItemFn, FnArg, PatType, ReturnType, PatIdent, Pat};

#[proc_macro_attribute]
pub fn auto_vec(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let mut new_sig = ast.sig.clone();
    new_sig.ident = format_ident!("{}_vec", new_sig.ident);

    let mut args = vec![];

    for i in new_sig.inputs.iter_mut() {
        if let FnArg::Typed(PatType {ref pat, ref mut ty, ..}) = i {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                let ty = &mut **ty;
                let a = TokenStream::from(quote! {Vec<#ty>});
                *ty = parse_macro_input!(a as syn::Type);
                args.push(format_ident!("{}", ident));
            }
        }
    }

    if let ReturnType::Type( .., ref mut second) = new_sig.output {
        let ty = &mut **second;
        let a = TokenStream::from(quote! {Vec<#ty>});
        *ty = parse_macro_input!(a as syn::Type);
    } else {
        let expanded = quote! { compile_error!("the return value can't be `()`"); };
        return expanded.into();
    }
    
    if args.len() == 0 {
        let expanded = quote! { compile_error!("can't have 0 args"); };
        return expanded.into();
    }
    
    let stmts = ast.block.stmts;

    let expanded = quote! {
        extern crate itertools;
        use itertools::izip;
        #new_sig {
            let mut l = vec![];
            #(l.push(#args.len());)*
            if l.iter().min() != l.iter().max() {
                panic!("the arrays are not of same length");
            }
            izip!(#(#args),*).map(|(#(#args),*)| {
                #(#stmts)*
            }).collect()
        }
    };
    expanded.into()
}