//use error_level::ErrorLevel;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ErrorLevel, attributes(level))]
pub fn log_level_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    dbg!(&ast);

    // Build the trait implementation
    impl_error_level_macro(&ast)
}

enum Level {
    No,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

struct Attr {
    level: Level,
    ident: syn::Ident,
}

fn impl_error_level_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let variants = &unwrap_enum(data).variants;
     
    //save list of variants with a level attribute
    //for idents without attr call the error_level function
    //if error_level is undefined for that type the user will
    //get an error

    let gen = quote! {
        impl ErrorLevel for #name {
            fn error_level(&self) -> log::Level {
                //for each attr add a case that makes the report
                println!("Hello, Macro! My name is {}!", stringify!(#name));
                log::Level::Warn
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
