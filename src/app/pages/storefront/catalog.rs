//! Product catalog listing page.

use crate::server::products::list_active_products;
use leptos::prelude::*;

/// Displays all active products in a responsive grid.
#[component]
pub fn CatalogPage() -> impl IntoView {
    let products = Resource::new(|| (), |_| list_active_products());

    view! {
        <section class="mx-auto max-w-7xl px-4 py-12">
            <h1 class="text-3xl font-bold">"Product Catalog"</h1>

            <Suspense fallback=move || view! { <p class="mt-4 text-gray-500">"Loading products…"</p> }>
                {move || products.get().map(|result| match result {
                    Ok(items) if items.is_empty() => view! {
                        <p class="mt-8 text-gray-500">"No products available yet."</p>
                    }.into_any(),
                    Ok(items) => view! {
                        <div class="mt-8 grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                            {items.into_iter().map(|p| {
                                let href = format!("/product?id={}", p.id);
                                let img = p.image_url.clone().unwrap_or_else(|| "No image".to_string());
                                let name = p.name.clone();
                                let price = format!("{} {:.2}", p.currency, p.price_cents as f64 / 100.0);
                                view! {
                                    <a href=href
                                       class="block rounded-xl border bg-white p-4 shadow-sm transition hover:shadow-md">
                                        <div class="h-40 rounded bg-gray-100 flex items-center justify-center text-gray-400">
                                            {img}
                                        </div>
                                        <h3 class="mt-3 font-semibold">{name}</h3>
                                        <p class="text-sm text-gray-500">{price}</p>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any(),
                    Err(e) => view! {
                        <p class="mt-4 text-red-600">{format!("Error loading products: {e}")}</p>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}
