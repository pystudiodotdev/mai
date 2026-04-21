//! Footer component.

use leptos::prelude::*;

/// Site-wide footer.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 py-6 text-center text-sm text-gray-400">
            <p>"© 2026 mai — AI-Integrated E-Commerce. Built with Rust & Leptos."</p>
        </footer>
    }
}
