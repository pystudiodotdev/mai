//! Top navigation bar component.

use leptos::prelude::*;

/// Site-wide navigation bar.
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="bg-brand-700 text-white shadow-md">
            <div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-3">
                // Brand
                <a href="/" class="text-xl font-bold tracking-wide">"mai"</a>

                // Links
                <div class="flex items-center gap-6 text-sm">
                    <a href="/catalog" class="hover:text-brand-200">"Catalog"</a>
                    <a href="/admin" class="hover:text-brand-200">"Admin"</a>
                    <a href="/login" class="rounded bg-white/10 px-3 py-1 hover:bg-white/20">
                        "Sign In"
                    </a>
                    <a href="/register" class="rounded bg-brand-500 px-3 py-1 hover:bg-brand-400">
                        "Register"
                    </a>
                </div>
            </div>
        </nav>
    }
}
