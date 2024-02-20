use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click me: " {count}</button>
        <ContextIsland>
            <LastValue/>
        </ContextIsland>
    }
}

#[island]
fn ContextIsland(children: Children) -> impl IntoView {
    let data = create_resource(
        || (),
        |()| async move { get_data().await.unwrap_or(Vec::new()) },
    );
    provide_context(data);
    children()
}

#[component]
fn LastValue() -> impl IntoView {
    let resource = use_context::<Resource<(), Vec<f64>>>();
    let value = move || {
        resource
            .and_then(|data| data.get())
            .and_then(|data| data.last().cloned())
            .unwrap_or(-1.)
    };

    view! {
        <Transition fallback=move || "~~">
            <p>"Last value: " {move || value()}</p>
        </Transition>
    }
}

#[server(GetPlotData, "/api")]
pub async fn get_data() -> Result<Vec<f64>, ServerFnError> {
    Ok(vec![0., 1., 2., 3., 4.])
}
