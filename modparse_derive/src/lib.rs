#![crate_type = "proc-macro"]
#![recursion_limit = "192"]
#![feature(trace_macros)]
#![feature(log_syntax)]


extern crate syn;
extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_derive(ModParse, attributes(module))]
pub fn parse(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let mut ast = syn::parse_derive_input(&source)
        .expect("failed to parse rust syntax");
    let gen = impl_parse(&mut ast);
    gen.parse()
        .expect("failed to serialize into rust syntax")
}

//fn filter_attrs(input: TokenStream) -> T

fn impl_parse(ast: &mut syn::DeriveInput) -> quote::Tokens {
    use syn::{Body, VariantData};

    let ref mut variants = match ast.body {
        Body::Struct(VariantData::Struct(ref vars)) => vars,
        _ => panic!("#[derive(Parse)] is only defined for braced structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let atmatch = variants.iter()
        .filter_map(|field| {
        	let field_name = field.ident.as_ref().clone();

        	let ret = if field.attrs.iter().any(|attr| attr.name() == "param") {
        		Some( quote! { stringify!(params.insert(stringify!(#field_name), & self. #field_name)) } )
        	} else if field.attrs.iter().any(|attr| attr.name() == "module") {
        		Some ( quote! { self.delegate._modules.insert(stringify!(#field_name), self. #field_name .clone()) } )
        	} else { None };
        	//field.attrs.retain(|ref attr| !names.contains(&attr.name()) );
    		ret
        });
    //println!("{:?}\n\n {:?}", atmatch, name);
    //println!("ast.ident: {:?}, {:?}", ast.ident, where_clause);    
    let foo = quote! {
        impl #impl_generics ModuleStruct for Rc< #name  #ty_generics > #where_clause {
            fn init_module(&self) {
            	self.delegate._name = stringify!(#name);
            	//let &mut modules = &self.delegate._modules;
//            	let &mut params = self.module._params;
				#(#atmatch);* 
				;
            }
        }
    };
    println!("parse is {}", foo);
    foo
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}