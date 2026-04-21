//! Admin dashboard overview page.

use leptos::prelude::*;

/// Admin dashboard landing page with navigation to sub-sections.
#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-5xl px-4 py-12">
            <h1 class="text-3xl font-bold">"Admin Dashboard"</h1>
            <p class="mt-2 text-gray-500">"Manage your store from this control panel."</p>

            <div class="mt-8 grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                <AdminCard href="/admin/products" title="Products" description="Add, edit, or remove products and manage inventory."/>
                <AdminCard href="/admin/users" title="Users" description="View registered users and manage roles."/>
                <AdminCard href="#" title="Categories" description="Organise products into categories."/>
                <AdminCard href="#" title="Orders" description="View and process customer orders. (Coming soon)"/>
                <AdminCard href="#" title="AI Insights" description="Recommendations and demand forecasting. (Coming soon)"/>
                <AdminCard href="#" title="Settings" description="Site-wide configuration. (Coming soon)"/>
            </div>
        </section>
    }
}

/// Dashboard navigation card.
#[component]
fn AdminCard(href: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <a href=href class="block rounded-xl border bg-white p-6 shadow-sm transition hover:shadow-md">
            <h3 class="text-lg font-semibold text-brand-700">{title}</h3>
            <p class="mt-1 text-sm text-gray-500">{description}</p>
        </a>
    }
}
