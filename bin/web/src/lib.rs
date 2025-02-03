#![allow(non_snake_case)]

use cfg_if::cfg_if;

pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use sqlx::SqlitePool;
    use leptos::prelude::*;

    #[derive(Debug, Clone)]
    pub struct AppState {
        pub pool:           SqlitePool,
        pub leptos_options: LeptosOptions,
    }

    impl FromRef<AppState> for LeptosOptions {
        fn from_ref(app_state: &AppState) -> Self { app_state.leptos_options.clone() }
    }
}
}
