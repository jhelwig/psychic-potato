use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        body::Body as AxumBody,
        extract::State,
        http::Request,
        response::{
            IntoResponse,
            Response,
        },
        routing::get,
        Router,
    };
    use leptos::{
        context::provide_context,
        logging::log,
        prelude::*,
    };
    use leptos_axum::{
        generate_route_list,
        handle_server_fns_with_context,
        LeptosRoutes,
    };
    use sqlx::sqlite::SqlitePoolOptions;

    use web::{AppState, app::*};

    #[tokio::main]
    async fn main() {
        async fn server_fn_handler(
            State(app_state): State<AppState>,
            request: Request<AxumBody>,
        ) -> impl IntoResponse {
            log!("Request: {:?}", request);

            handle_server_fns_with_context(
                move || {
                    provide_context(app_state.pool.clone());
                },
                request,
            )
            .await
        }

        async fn leptos_routes_handler(
            State(app_state): State<AppState>,
            req: Request<AxumBody>,
        ) -> Response {
            let handler = leptos_axum::render_app_to_stream_with_context(
                move || {
                    provide_context(app_state.pool.clone());
                },
                move || shell(app_state.leptos_options.clone()),
            );
            handler(req).await.into_response()
        }

        let pool = SqlitePoolOptions::new()
            .connect("sqlite:league.db?mode=rwc")
            .await
            .expect("Could not make sqlite pool");

        sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let app_state = AppState {
            pool,
            leptos_options,
        };

        let app = Router::new()
            .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
            .with_state(app_state);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service()).await.unwrap();
    }
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
