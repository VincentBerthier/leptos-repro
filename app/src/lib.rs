use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-islands-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
        <I18nContextProvider>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </I18nContextProvider>
        </Router>
    }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
    let i18n = use_i18n();
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>{t!(i18n, click_me, count)}</button>
    }
}
