use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use leptos::*;
use leptos_chartistry::{AspectRatio, Chart, Line, Series, Tooltip};
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

leptos_i18n::load_locales!();

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
    let resource = create_resource(|| (), |()| async move { get_data().await });
    let data =
        Signal::derive(move || resource().map_or(Vec::new(), |data| data.unwrap_or_default()));
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let series = Series::new(|(x, _)| *x).line(Line::new(|(_, y)| *y));
    let plot = Signal::derive(move || {
        let series = series.clone();
        view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_ratio(300., 120.)
            series
            data
            tooltip=Tooltip::left_cursor()
        />
        }
    });
}

#[server(GetPlotData, "/api")]
pub async fn get_data() -> Result<Vec<(f64, f64)>, ServerFnError> {
    Ok(vec![(0., 0.), (1., 1.), (2., 2.), (3., 3.), (4., 4.)])
}
