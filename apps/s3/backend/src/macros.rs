#[macro_export]
macro_rules! from_req_extension {
  ($type:ty) => {
    impl<S: Sync> axum::extract::FromRequestParts<S> for $type {
      type Rejection = std::convert::Infallible;

      async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
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
  };
}

#[macro_export]
macro_rules! state_trait {
  (async fn $name:ident($($arg:tt)*) -> Self { $($body:tt)* }) => {
    #[allow(non_camel_case_types)]
    pub trait $name {
      async fn $name(self, config: &$crate::config::Config) -> Self;
    }

    impl $name for axum::Router {
      async fn $name($($arg)*) -> Self {
        $($body)*
      }
    }
  };
}

#[macro_export]
macro_rules! collect_state {
  ($($mod:ident),*) => {
    trait CollectedState {
      async fn state(
        self,
        config: &$crate::config::Config,
      ) -> Self;
    }

    impl CollectedState for axum::Router {
      async fn state(
        self,
        config: &$crate::config::Config,
      ) -> Self {
        $(
          use $mod::$mod;
        )*
        self
          $(
            .$mod(config).await
          )*
      }
    }
  };
}
