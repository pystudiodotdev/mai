//! Admin product management page.

use crate::server::admin::admin_list_products;
use leptos::prelude::*;

/// Lists all products (including inactive) for admin management.
#[component]
pub fn AdminProducts() -> impl IntoView {
    let products = Resource::new(|| (), |_| admin_list_products());

    view! {
        <section class="mx-auto max-w-7xl px-4 py-12">
            <div class="flex items-center justify-between">
                <h1 class="text-3xl font-bold">"Manage Products"</h1>
                <a href="/admin" class="text-sm text-brand-600 hover:underline">"← Dashboard"</a>
            </div>

            <Suspense fallback=move || view! { <p class="mt-4 text-gray-500">"Loading…"</p> }>
                {move || products.get().map(|result| match result {
                    Ok(items) if items.is_empty() => view! {
                        <p class="mt-8 text-gray-500">"No products found."</p>
                    }.into_any(),
                    Ok(items) => view! {
                        <div class="mt-6 overflow-x-auto">
                            <table class="min-w-full text-left text-sm">
                                <thead class="border-b font-medium text-gray-600">
                                    <tr>
                                        <th class="px-4 py-2">"Name"</th>
                                        <th class="px-4 py-2">"SKU"</th>
                                        <th class="px-4 py-2">"Price"</th>
                                        <th class="px-4 py-2">"Stock"</th>
                                        <th class="px-4 py-2">"Active"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {items.into_iter().map(|p| {
                                        let name = p.name.clone();
                                        let sku = p.sku.clone().unwrap_or_default();
                                        let price = format!("{} {:.2}", p.currency, p.price_cents as f64 / 100.0);
                                        let active = if p.is_active { "✓" } else { "✗" };
                                        view! {
                                            <tr class="border-b hover:bg-gray-50">
                                                <td class="px-4 py-2 font-medium">{name}</td>
                                                <td class="px-4 py-2 text-gray-500">{sku}</td>
                                                <td class="px-4 py-2">{price}</td>
                                                <td class="px-4 py-2">{p.stock_qty}</td>
                                                <td class="px-4 py-2">{active}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    }.into_any(),
                    Err(e) => view! {
                        <p class="mt-4 text-red-600">{format!("Error: {e}")}</p>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}
