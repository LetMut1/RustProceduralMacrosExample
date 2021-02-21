extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::FieldsNamed;
use syn::Ident;
use syn::parse_macro_input;
use syn::parse::Parser;

#[proc_macro_derive(As_pg_connection_manager_holder)]
pub fn as_pg_connection_manager_holder(input: TokenStream) -> TokenStream {
  let mut derive_input: DeriveInput = parse_macro_input!(input);
  let ident: &Ident = &derive_input.ident;
  let data: &mut Data = &mut derive_input.data;
  match data {
    Data::Struct(ref mut data_struct) => {
      match &mut data_struct.fields {
        Fields::Named(ref mut fields_named) => {
          fields_named.named.push(
            (Field::parse_named).parse2(
              quote! { 
                pg_connection_manager: &'b crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager
              }
            ).unwrap()
          )
      },
        _ => panic!("Custom Derive: not implemented")
      };
    },
    _ => panic!("Custom Derive: not implemented")
  };

  return TokenStream::from(     // TODO make from proc_macro2
    quote! {
      #derive_input,
      impl #ident {
        pub fn new(pg_connection_manager: &'b crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager) -> Self {
          return Self {
              pg_connection_manager
          };
        }
  
        fn get_pg_connection_manager(&'a self) -> &'b crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager {
          return self.pg_connection_manager;
        }
      }
    }
  );
}