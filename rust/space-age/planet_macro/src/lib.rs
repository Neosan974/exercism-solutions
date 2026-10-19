use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitFloat, parse_macro_input};

#[proc_macro_attribute]
pub fn planet_ratio(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. On parse l'argument de l'attribut comme un nombre à virgule flottante (f64)
    let ratio = parse_macro_input!(attr as LitFloat);

    // 2. On parse l'élément sur lequel est posé l'attribut (doit être une struct)
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident; // Récupère le nom de la struct (ex: Mercury)

    // 3. On génère le code de la struct originale + l'implémentation du trait Planet
    let expanded = quote! {
        // On réaffiche la structure d'origine
        #input_struct

        // On génère l'implémentation
        impl Planet for #struct_name {
            fn get_ratio_from_earth_year() -> f64 {
                #ratio
            }
        }
    };

    // 4. On convertit le tout en TokenStream pour le compilateur
    expanded.into()
}
