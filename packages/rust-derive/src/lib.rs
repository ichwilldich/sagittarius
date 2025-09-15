use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(FromReqExtension)]
pub fn from_req(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as ItemStruct);

  let mut input_generics = input.clone();
  input_generics
    .generics
    .params
    .push(syn::parse_quote! { FromRequestGeneric: Sync });
  let (impl_generics, _, _) = input_generics.generics.split_for_impl();

  let name = input.ident;
  let (_, type_generics, where_clause) = input.generics.split_for_impl();

  quote! {
    impl #impl_generics axum::extract::FromRequestParts<FromRequestGeneric> for #name #type_generics #where_clause {
      type Rejection = std::convert::Infallible;

      async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &FromRequestGeneric,
      ) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;

        Ok(
          parts
            .extract::<axum::Extension<Self>>()
            .await
            .expect(
              format!(
                "Should not fail. Did you add Extension({}) to your app?",
                std::any::type_name::<Self>()
              )
              .as_str(),
            )
            .0,
        )
      }
    }
  }
  .into()
}
