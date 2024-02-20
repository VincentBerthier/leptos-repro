use std::time::Duration;

use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_query::{
    create_query, use_query, QueryOptions, QueryResult, QueryScope, RefetchFn, ResourceOption,
};
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::*;
use tracing::{debug, instrument, Instrument};

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Provides Query Client for entire app.
    leptos_query::provide_query_client();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-islands-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <LeptosQueryDevtools/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let QueryResult { data, .. } = data_query().use_query(|| ());
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let points = move || {
        data.get().map(|data| {
            view! {
                <For each=move || data.clone().into_iter().enumerate() key=move |(i, _)| *i let:point>
                    <p>"Point at {" {point.1.0} "," {point.1.1} "}"</p>
                </For>
            }
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Transition fallback=move || view! { <p>"Loading data"</p> }>
        {points}
        </Transition>
    }
}

#[instrument]
#[server(GetPlotData, "/api")]
pub async fn get_data() -> Result<Vec<(f64, f64)>, ServerFnError> {
    debug!("getting data");
    Ok(vec![(0., 0.), (1., 1.), (2., 2.), (3., 3.), (4., 4.)])
}

#[instrument]
pub fn data_query() -> QueryScope<(), Vec<(f64, f64)>> {
    create_query(
        |_| {
            async move {
                debug!("querying data");
                get_data().await.unwrap_or_default()
            }
            .in_current_span()
        },
        QueryOptions {
            stale_time: Some(Duration::from_secs(30)),
            gc_time: Some(Duration::from_secs(60)),
            ..Default::default()
        },
    )
}
