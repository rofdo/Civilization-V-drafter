use serde::{Deserialize, Serialize};
use quote::quote;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
enum Bias {
    Coastal,
    // also may be called Hills
    #[serde(alias = "Hills")]
    Hill,
    Jungle,
    Desert,
    Grassland,
    Tundra,
    Forest,
    AvoidJungle,
    AvoidDesert,
    AvoidTundra,
    None,
}

trait Civ {
    fn name(&self) -> &str;
    fn leader(&self) -> &str;
    fn bias(&self) -> Vec<Bias>;
}

use proc_macro::TokenStream;
#[proc_macro_derive(Civ)]
pub fn derive_leader(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_civ(&ast)
}

fn impl_civ(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Civ for #name {
            fn name(&self) -> &str {
                &self.name
            }
            fn leader(&self) -> &str {
                &self.leader
            }
            fn bias(&self) -> Vec<Bias> {
                self.bias
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
