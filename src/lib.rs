
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Expr};

#[proc_macro_derive(Zerohex)]
pub fn derive_zerohex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut array_size = None;

    // Ensure the struct has exactly one unnamed field of type [u8; N]
    let mut valid = false;
    if let Data::Struct(data) = &input.data {
        if let Fields::Unnamed(fields) = &data.fields {
            if fields.unnamed.len() == 1 {
                let field = &fields.unnamed.first().unwrap().ty;
                if let Type::Array(arr) = field {
                    if let Type::Path(path) = &*arr.elem {
                        if path.path.is_ident("u8") {
                            if let Expr::Lit(expr_lit) = &arr.len {
                                if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                                    array_size = Some(lit_int.base10_parse::<usize>().unwrap());
                                    valid = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !valid {
        return syn::Error::new_spanned(name, "Zerohex can only be derived for structs with a single [u8; N] field.")
            .to_compile_error()
            .into();
    }

    let n = array_size.expect("Failed to extract array size");
    let n_char = n * 2;
    let n_char_prefixed = n_char + 2;

    let expanded = quote! {
        use serde::{Serialize, Deserialize, de::Error};
        use thiserror::Error;
        use hex::{FromHex, FromHexError};
        use std::{str::FromStr};

        impl std::str::FromStr for #name {
            type Err = hex::FromHexError;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.len() {
                    #n_char => Ok(Self(<[u8;#n]>::from_hex(s).unwrap())),
                    #n_char_prefixed => if s[..2].to_lowercase() == "0x" {
                        Ok(Self(<[u8;#n]>::from_hex(&s[2..])?))
                    } else {
                        Err(hex::FromHexError::InvalidStringLength)
                    },
                    _ => Err(hex::FromHexError::InvalidStringLength),
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "0x{}", hex::encode(self.0))
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!(#name))
                .field(&format!("{}", self))
                .finish()
            }
        }

        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&format!("{}", self))
            }
        }

        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(#name::from_str(&String::deserialize(deserializer)?).map_err(D::Error::custom)?)
            }
        }
    };

    // panic!("{}", expanded);
    TokenStream::from(expanded)
}