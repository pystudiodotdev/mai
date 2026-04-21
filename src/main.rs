//! Server entry point for the mai application.
//!
//! When compiled with the `ssr` feature this binary starts an Axum server that
//! serves the Leptos application with server-side rendering, connects to
//! PostgreSQL, and runs pending migrations on startup.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mai::app::*;
    use mai::server::db::{create_pool, run_migrations};

    // Initialise tracing
    tracing_subscriber::fmt::init();

    // Database setup
    let pool = create_pool().await;
    run_migrations(&pool).await;
    log!("Database connected and migrations applied.");

    // Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes defined in the Leptos App
    let routes = generate_route_list(App);

    // Build the Axum router
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Make the PgPool available to server functions via `expect_context`.
        .layer(axum::Extension(pool));

    // Start the server
    log!("Listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Client-side builds do not need a `main` — hydration is handled by `lib.rs`.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
