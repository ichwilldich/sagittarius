#[macro_export]
macro_rules! router_extension {
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
macro_rules! collect_router_extensions {
  ($name:ident; $($extension:ident),+) => {
    trait $name {
      async fn state(self, config: &$crate::config::Config) -> Self;
    }

    impl $name for axum::Router {
      async fn state(self, config: &$crate::config::Config) -> Self {
        self
          $(
            .$extension(config).await
          )*
      }
    }
  };
}
