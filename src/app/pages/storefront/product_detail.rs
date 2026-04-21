//! Product detail page.

use leptos::prelude::*;

/// Shows full details for a single product.
///
/// Expects a `?id=<uuid>` query parameter. In a more advanced setup this would
/// use a path param via `leptos_router::ParamSegment`.
#[component]
pub fn ProductDetailPage() -> impl IntoView {
    // For simplicity, we'll show a placeholder that loads a product by id.
    // A real implementation would extract the id from the route params.

    view! {
        <section class="mx-auto max-w-3xl px-4 py-12">
            <h1 class="text-3xl font-bold">"Product Detail"</h1>
            <p class="mt-4 text-gray-500">
                "Select a product from the catalog to see its details here. "
                "This page will be enhanced with dynamic route parameters."
            </p>

            <a href="/catalog" class="mt-6 inline-block text-brand-600 hover:underline">
                "← Back to Catalog"
            </a>
        </section>
    }
}
