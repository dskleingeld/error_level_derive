//use error_level::ErrorLevel;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, punctuated::Punctuated, Variant, token::Comma, Attribute};

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

#[derive(Debug)]
struct Marked {
    level: Level,
    variant_id: syn::Ident,
}

fn log_level(v: &Variant) -> Option<Level> { 
    fn has_level_path(m: &syn::MetaList) -> bool {
        if let Some(ident) = m.path.get_ident() {
            ident == "level"
        } else {
            false
        }
    }
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
        if let Some(level) = log_level(v){
            let variant_id = v.ident.clone();
            marked.push(Marked {
                level,
                variant_id
            });
        }
    }
    marked
}

fn impl_error_level_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let variants = &unwrap_enum(data).variants;
    //save list of variants with a level attribute
    let marked = marked_variants(variants);
    
    //for idents without attr call the error_level function
    //if error_level is undefined for that type the user will
    //get an error

    let gen = quote! {
        impl ErrorLevel for #name {
            fn error_level(&self) -> Option<log::Level> {
                //for each attr add a case that makes the report
                println!("Hello, Macro! My name is {}!", stringify!(#name));
                Some(log::Level::Warn)
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
