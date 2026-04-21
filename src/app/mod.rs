//! Application root — shell, routing, and top-level layout.

pub mod components;
pub mod pages;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use self::components::layout::Layout;
use self::pages::{
    admin::{AdminDashboard, AdminProducts, AdminUsers},
    auth::{LoginPage, RegisterPage},
    home::HomePage,
    storefront::{CatalogPage, ProductDetailPage},
};

/// Outer HTML shell rendered on the server before hydration kicks in.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="min-h-screen bg-gray-50 text-gray-900">
                <App/>
            </body>
        </html>
    }
}

/// Root application component — sets up context, styles, and the router.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/mai.css"/>
        <Title text="mai — AI E-Commerce"/>

        <Router>
            <Layout>
                <main class="flex-1">
                    <Routes fallback=|| view! { <p class="p-8 text-center text-red-600">"404 — Page not found."</p> }.into_view()>
                        // Public routes
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("catalog") view=CatalogPage/>
                        <Route path=StaticSegment("product") view=ProductDetailPage/>

                        // Auth routes
                        <Route path=StaticSegment("login") view=LoginPage/>
                        <Route path=StaticSegment("register") view=RegisterPage/>

                        // Admin routes (protected)
                        <Route path=StaticSegment("admin") view=AdminDashboard/>
                        <Route path=StaticSegment("admin/products") view=AdminProducts/>
                        <Route path=StaticSegment("admin/users") view=AdminUsers/>
                    </Routes>
                </main>
            </Layout>
        </Router>
    }
}
