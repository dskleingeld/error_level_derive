//use error_level::ErrorLevel;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{self, spanned::Spanned, punctuated::Punctuated, Variant, token::Comma, Attribute, Fields};

#[proc_macro_derive(ErrorLevel, attributes(level))]
pub fn log_level_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_error_level_macro(&ast)
}

#[derive(Debug)]
enum Level {
    No,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn from_ident(id: &syn::Ident) -> Self {
        match id.to_string().as_str() {
            "No" => Self::No,
            "Trace" => Self::Trace,
            "Debug" => Self::Debug,
            "Info" => Self::Info,
            "Warn" => Self::Warn,
            "Error" => Self::Error,
            _ => panic!("options are only: No, Trace, Debug, Info, Warn or Error"),
        }
    }
}

impl quote::ToTokens for Level {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = match self {
            Self::No => quote! {None},
            Self::Trace => quote! {Some(log::Level::Trace)},
            Self::Debug => quote! {Some(log::Level::Trace)},
            Self::Info => quote! {Some(log::Level::Trace)},
            Self::Warn => quote! {Some(log::Level::Trace)},
            Self::Error => quote! {Some(log::Level::Trace)},
        };
        tokens.extend(token);
    }
}

#[derive(Debug)]
struct Marked {
    level: Level,
    variant_id: syn::Ident,
}

fn has_level_path(m: &syn::MetaList) -> bool {
    if let Some(ident) = m.path.get_ident() {
        ident == "level"
    } else {
        false
    }
}

fn with_log_level(v: &Variant) -> Option<Level> { 
    fn unwrap_meta(n: &syn::NestedMeta) -> &syn::Meta {
        if let syn::NestedMeta::Meta(m) = n {
            return m;
        }
        panic!("nested argument list should not be a rust literal but a structured meta item");
    }

    for a in &v.attrs {
        let m = a.parse_meta().unwrap();
        if let syn::Meta::List(list) = m { 
            if !has_level_path(&list){continue;}
            let nested = list.nested.first().unwrap();
            let meta = unwrap_meta(&nested);
            let ident = meta.path().get_ident().unwrap();
            return Some(Level::from_ident(ident));
        }
    }
    None
}

fn marked_variants(variants: &Punctuated<Variant, Comma>) -> Vec<Marked> {
    let mut marked = Vec::new();
    for v in variants { 
        if let Some(level) = with_log_level(v){
            let variant_id = v.ident.clone();
            marked.push(Marked {
                level,
                variant_id
            });
        }
    }
    marked
}

#[derive(Debug)]
struct WithInnError {
    inner_id: syn::Ident,
    variant_id: syn::Ident,
}

enum Valid {
    Yes(syn::Ident),
    No(proc_macro2::Span),
}

fn is_valid_inner(ty: &syn::Type) -> Valid {
    match ty {
        syn::Type::Path(p) => Valid::Yes(p.path.get_ident().unwrap().clone()),
        syn::Type::Reference(r) =>
            if let syn::Type::Path(p) = &*r.elem {
                Valid::Yes(p.path.get_ident().unwrap().clone())
            } else {
                Valid::No(r.span())
            },
        _ => Valid::No(ty.span())
    }
}

fn has_inner(v: &Variant) -> Option<&syn::Type> { 
    for a in &v.attrs {
        let m = a.parse_meta().unwrap();
        if let syn::Meta::List(list) = m { 
            if has_level_path(&list){return None;}
        }
    }

    if let Fields::Unnamed(syn::FieldsUnnamed {ref unnamed, ..}) = v.fields {
        let ty = &unnamed.first()?.ty;
        Some(ty)
    } else {
        None
    }
}

// fn with_inner_error(variants: &Punctuated<Variant, Comma>) -> Vec<WithInnError> {
//     let mut w_inn_err = Vec::new();
//     for v in variants { 
//         if let Some(inner_id) = could_have_inner_err(v){
//             let variant_id = v.ident.clone();
//             w_inn_err.push(WithInnError {
//                 inner_id,
//                 variant_id
//             });
//         }
//     }
//     w_inn_err
// }

fn extract_variants(variants: &Punctuated<Variant, Comma>) -> (Vec<Marked>, Vec<WithInnError>, Vec<proc_macro2::TokenStream>) {
    let mut marked = Vec::new();
    let mut w_inner = Vec::new();
    let mut errs = Vec::new();

    for v in variants {
        if let Some(level) = with_log_level(v){
            let variant_id = v.ident.clone();
            marked.push(Marked {
                level,
                variant_id
            });
        } else if let Some(inner) = has_inner(v){
            match is_valid_inner(inner) {
                Valid::Yes(inner_id) => {    
                    let variant_id = v.ident.clone();
                    w_inner.push(WithInnError {
                        inner_id,
                        variant_id
                    });
                },
                Valid::No(span) => {
                    errs.push(quote_spanned! {
                        span =>
                        compile_error!("Need 'Level' attribute, variant content can not get an 'ErrorLevel' trait implementation");
                    });
                },
            }
        } else {
            errs.push(quote_spanned! {
                v.span() =>
                compile_error!("Need 'Level' attribute");
            })
        }
    }
    (marked, w_inner, errs)
}

fn impl_error_level_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let variants = &unwrap_enum(data).variants;
    let (marked, w_inner, errs) = extract_variants(variants);

    //save list of variants with a level attribute
    let level_with_attr = marked.iter().map(|m| &m.level);
    let ident_with_attr = marked.iter().map(|m| &m.variant_id);

    //for idents without attr call the error_level function
    //if error_level is undefined for that type the user will
    let ident_no_attr = w_inner.iter().map(|m| &m.variant_id);

    let gen = quote! {
        impl ErrorLevel for #name {
            fn error_level(&self) -> Option<log::Level> {
                match self {
                    #(#name::#ident_with_attr => #level_with_attr,)*
                    #(#name::#ident_no_attr(inn_err) => inn_err.error_level(),)*
                }
                #(#errs)*;
            }
        }
    };
    gen.into()
}

fn unwrap_enum(data: &syn::Data) -> &syn::DataEnum {
    if let syn::Data::Enum(v) = data {
        return v;
    } else {
        panic!("can only implement error level on enums");
    }
}
