//! Page layout wrapper — nav + content + footer.

use leptos::prelude::*;

use super::footer::Footer;
use super::nav::Nav;

/// Full-page layout that wraps every route.
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col">
            <Nav/>
            {children()}
            <Footer/>
        </div>
    }
}
