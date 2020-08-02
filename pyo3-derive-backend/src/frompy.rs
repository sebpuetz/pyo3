use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Fields, Type, DeriveInput, DataEnum, DataStruct};

pub fn derive_enum(data_enum: &mut DataEnum) -> syn::Result<TokenStream> {
    let mut union_types = String::new();
    let mut names = Vec::new();
    for (i, var) in data_enum.variants.iter().enumerate() {
        let mut name = None;
        for attr in &var.attrs {
            let attr = attr.parse_meta()?;
            if attr.path().is_ident("rename") {
                if let syn::Meta::NameValue(ref nv) = attr {
                    match &nv.lit {
                        syn::Lit::Str(s) => { name = Some(s); },
                        _ => panic!(),
                    }
                }
            }
        }
        if let Some(name) = name {
            names.push(name.to_owned())
        } else {
            names.push(syn::LitStr::new(&var.ident.to_string(), var.span()))
        }
        //union_types.push_str(&var.ident.to_string());
        if i != data_enum.variants.len() - 1 {
           union_types.push_str(", ")
        }
    }
    println!("{}", union_types);
    Ok(TokenStream::new())
}

pub fn derive_struct(data_struct: &mut DataStruct) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}

pub fn derive_from_py(tokens: &mut DeriveInput) -> syn::Result<TokenStream> {
    let ident = Ident::new(&tokens.ident.to_string(), tokens.ident.span());
    //let mut var_extracts = Vec::new();
    //let mut union_types = String::new();
    match &mut tokens.data {
        syn::Data::Enum(en) => derive_enum(en),
        syn::Data::Struct(st) => derive_struct(st),
        union => panic!(),
    }
    // for (i, var) in tokens.variants.iter().enumerate() {
    //     // TODO allow rename for err-msg
    //     union_types.push_str(&var.ident.to_string());
    //     // TODO support named fields
    //     match var.fields {
    //         Fields::Unnamed(_) => (),
    //         _ => {
    //             return Err(syn::Error::new(
    //                 var.span(),
    //                 "Currently only NewType variants allowed.",
    //             ))
    //         }
    //     }
    //     let var_ident = &var.ident;
    //     // TODO allow variants with multiple fields
    //     if var.fields.len() != 1 {
    //         return Err(syn::Error::new(
    //             var.span(),
    //             "Currently only NewType variants allowed",
    //         ));
    //     }
    //     let ty: &Type = &var.fields.iter().next().unwrap().ty;
    //     if let Type::Reference(ty_ref) = ty {
    //         let elem = ty_ref.elem.as_ref();
    //         // TODO hard-coded ob
    //         // TODO #ident::#var_ident seems wrong
    //         var_extracts.push(quote!(
    //             if let Ok(ob) = #elem::try_from(ob) {
    //                 return Ok(#ident::#var_ident(ob));
    //             }
    //         ));
    //     } else if let Type::Path(_) = ty {
    //         var_extracts.push(quote!(
    //             if let Ok(ob) = ::pyo3::FromPyObject::extract(ob) {
    //                 return Ok(#ident::#var_ident(ob));
    //             }
    //         ))
    //     } else {
    //         return Err(syn::Error::new(ty.span(), "Expected reference"));
    //     }
    //
    //     if i != tokens.variants.len() - 1 {
    //         union_types.push_str(", ")
    //     }
    // }
    // let union = if tokens.variants.len() > 1 {
    //     format!("Union[{}]", union_types)
    // } else {
    //     union_types
    // };
    // Ok(quote!(
    //     impl<'source> ::pyo3::FromPyObject<'source> for #ident<'source> {
    //         fn extract(ob: &'source ::pyo3::PyAny) -> ::pyo3::PyResult<Self>  {
    //             #(#var_extracts);*;
    //             let type_name = ob.get_type().name();
    //             let err_msg = format!("Can't convert {} to {}", type_name, #union);
    //             Err(::pyo3::exceptions::PyTypeError::py_err(err_msg))
    //         }
    //     }
    // ))
}
