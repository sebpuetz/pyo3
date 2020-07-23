use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Fields, ItemEnum, Type};

pub fn build_wrapper_enum(tokens: &mut ItemEnum) -> syn::Result<TokenStream> {
    let ident = Ident::new(&tokens.ident.to_string(), tokens.ident.span());
    let mut var_extracts = Vec::new();
    let mut union_types = String::new();
    for (i, var) in tokens.variants.iter().enumerate() {
        // TODO allow rename for err-msg
        union_types.push_str(&var.ident.to_string());
        // TODO support named fields
        match var.fields {
            Fields::Unnamed(_) => (),
            _ => {
                return Err(syn::Error::new(
                    var.span(),
                    "Currently only NewType variants allowed.",
                ))
            }
        }
        let var_ident = &var.ident;
        // TODO allow variants with multiple fields
        if var.fields.len() != 1 {
            return Err(syn::Error::new(
                var.span(),
                "Currently only NewType variants allowed",
            ));
        }
        let ty: &Type = &var.fields.iter().next().unwrap().ty;
        if let Type::Reference(ty_ref) = ty {
            let elem = ty_ref.elem.as_ref();
            // TODO hard-coded ob
            // TODO #ident::#var_ident seems wrong
            var_extracts.push(quote!(
                if let Ok(ob) = #elem::try_from(ob) {
                    return Ok(#ident::#var_ident(ob));
                }
            ));
        } else if let Type::Path(_) = ty {
            var_extracts.push(quote!(
                if let Ok(ob) = ::pyo3::FromPyObject::extract(ob) {
                    return Ok(#ident::#var_ident(ob));
                }
            ))
        } else {
            return Err(syn::Error::new(ty.span(), "Expected reference"));
        }

        if i != tokens.variants.len() - 1 {
            union_types.push_str(", ")
        }
    }
    let union = if tokens.variants.len() > 1 {
        format!("Union[{}]", union_types)
    } else {
        union_types
    };
    Ok(quote!(
        impl<'source> ::pyo3::FromPyObject<'source> for #ident<'source> {
            fn extract(ob: &'source ::pyo3::PyAny) -> ::pyo3::PyResult<Self>  {
                #(#var_extracts);*;
                let type_name = ob.get_type().name();
                let err_msg = format!("Can't convert {} to {}", type_name, #union);
                Err(::pyo3::exceptions::PyTypeError::py_err(err_msg))
            }
        }
    ))
}
