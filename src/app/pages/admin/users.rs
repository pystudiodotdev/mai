//! Admin user management page.

use crate::server::admin::admin_list_users;
use leptos::prelude::*;

/// Lists all registered users for admin review.
#[component]
pub fn AdminUsers() -> impl IntoView {
    let users = Resource::new(|| (), |_| admin_list_users());

    view! {
        <section class="mx-auto max-w-7xl px-4 py-12">
            <div class="flex items-center justify-between">
                <h1 class="text-3xl font-bold">"Manage Users"</h1>
                <a href="/admin" class="text-sm text-brand-600 hover:underline">"← Dashboard"</a>
            </div>

            <Suspense fallback=move || view! { <p class="mt-4 text-gray-500">"Loading…"</p> }>
                {move || users.get().map(|result| match result {
                    Ok(items) if items.is_empty() => view! {
                        <p class="mt-8 text-gray-500">"No users found."</p>
                    }.into_any(),
                    Ok(items) => view! {
                        <div class="mt-6 overflow-x-auto">
                            <table class="min-w-full text-left text-sm">
                                <thead class="border-b font-medium text-gray-600">
                                    <tr>
                                        <th class="px-4 py-2">"Username"</th>
                                        <th class="px-4 py-2">"Email"</th>
                                        <th class="px-4 py-2">"Role"</th>
                                        <th class="px-4 py-2">"Joined"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {items.into_iter().map(|u| {
                                        let username = u.username.clone();
                                        let email = u.email.clone();
                                        let role = u.role.clone();
                                        let joined = u.created_at.format("%Y-%m-%d").to_string();
                                        view! {
                                            <tr class="border-b hover:bg-gray-50">
                                                <td class="px-4 py-2 font-medium">{username}</td>
                                                <td class="px-4 py-2 text-gray-500">{email}</td>
                                                <td class="px-4 py-2">{role}</td>
                                                <td class="px-4 py-2 text-gray-400">{joined}</td>
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
