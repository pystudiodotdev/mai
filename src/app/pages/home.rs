//! Landing / home page.

use leptos::prelude::*;

/// Public home page — hero section and value proposition.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-5xl px-4 py-20 text-center">
            <h1 class="text-5xl font-extrabold tracking-tight text-brand-700">
                "Welcome to mai"
            </h1>
            <p class="mx-auto mt-4 max-w-2xl text-lg text-gray-600">
                "An AI-integrated e-commerce platform built for performance, reliability, and intelligent automation."
            </p>

            <div class="mt-10 flex justify-center gap-4">
                <a href="/catalog"
                   class="rounded-lg bg-brand-600 px-6 py-3 text-white shadow hover:bg-brand-500">
                    "Browse Catalog"
                </a>
                <a href="/register"
                   class="rounded-lg border border-brand-600 px-6 py-3 text-brand-600 hover:bg-brand-50">
                    "Get Started"
                </a>
            </div>

            // Feature highlights
            <div class="mt-16 grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
                <FeatureCard title="AI Integration" description="Intelligent recommendations and demand forecasting."/>
                <FeatureCard title="Omnichannel" description="Unified experience across every sales channel."/>
                <FeatureCard title="Comprehensive Tools" description="Full lifecycle management for your store."/>
                <FeatureCard title="Zero-Cost Connect" description="Marketplace integrations at no extra charge."/>
            </div>
        </section>
    }
}

/// A small card highlighting a single feature.
#[component]
fn FeatureCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="rounded-xl border bg-white p-6 shadow-sm">
            <h3 class="text-lg font-semibold text-brand-700">{title}</h3>
            <p class="mt-2 text-sm text-gray-500">{description}</p>
        </div>
    }
}
