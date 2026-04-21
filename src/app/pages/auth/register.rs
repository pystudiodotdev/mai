//! Registration page component.

use leptos::prelude::*;

/// Registration form page.
#[component]
pub fn RegisterPage() -> impl IntoView {
    let register_action = ServerAction::<RegisterUser>::new();
    let value = register_action.value();

    view! {
        <section class="mx-auto max-w-md px-4 py-16">
            <h1 class="text-3xl font-bold text-center">"Create Account"</h1>

            <ActionForm action=register_action attr:class="mt-8 space-y-4">
                <div>
                    <label for="username" class="block text-sm font-medium">"Username"</label>
                    <input type="text" name="input[username]" required
                           class="mt-1 w-full rounded border px-3 py-2 focus:border-brand-500 focus:ring-brand-500"/>
                </div>
                <div>
                    <label for="email" class="block text-sm font-medium">"Email"</label>
                    <input type="email" name="input[email]" required
                           class="mt-1 w-full rounded border px-3 py-2 focus:border-brand-500 focus:ring-brand-500"/>
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium">"Password"</label>
                    <input type="password" name="input[password]" required
                           class="mt-1 w-full rounded border px-3 py-2 focus:border-brand-500 focus:ring-brand-500"/>
                </div>
                <button type="submit"
                        class="w-full rounded bg-brand-600 py-2 text-white hover:bg-brand-500">
                    "Register"
                </button>
            </ActionForm>

            // Display result message
            {move || value.get().map(|result| match result {
                Ok(auth) => view! {
                    <p class=if auth.success { "mt-4 text-green-600" } else { "mt-4 text-red-600" }>
                        {auth.message}
                    </p>
                }.into_any(),
                Err(e) => view! {
                    <p class="mt-4 text-red-600">{format!("Error: {e}")}</p>
                }.into_any(),
            })}

            <p class="mt-6 text-center text-sm text-gray-500">
                "Already have an account? "
                <a href="/login" class="text-brand-600 hover:underline">"Sign In"</a>
            </p>
        </section>
    }
}

// Re-export the server fn type so ActionForm can find it.
use crate::server::auth::RegisterUser;
