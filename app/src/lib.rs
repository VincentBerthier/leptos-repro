use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use chrono::{DateTime, Utc};
use leptos::serde::{Deserialize, Serialize};
use leptos::*;
use leptos_chartistry::{AspectRatio, Chart, Colour, Line, SequentialGradient, Series, Tooltip};
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
                        <Route path="/" view=Nav>
                            <Route path="" view=HomePage/>
                        </Route>
                    </Routes>
                </main>
            </I18nContextProvider>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="#">{ti!(Home, home)}</a>
            " "
            <a href="#">{ti!(Other, other)}</a>
        </nav>
        <Outlet/>
    }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
    let i18n = use_i18n();
    let data = create_resource(
        || (),
        |()| async move { get_data().await.map_or_else(|_| Vec::new(), |data| data) },
    );
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let series = Series::new(|data: &TimePoint| cast_date(data.time))
        .line(Line::new(|data: &TimePoint| data.price).with_gradient(GRADIENT));

    let series = series.clone();
    let tooltip = Tooltip::left_cursor().with_class("bg-background-1 text-white rounded-xl");

    view! {
        <button on:click=on_click>{t!(i18n, click_me, count)}</button>
        <Transition fallback=|| view! { <p>"loading data"</p> }>
            <Chart
                aspect_ratio=AspectRatio::from_outer_ratio(600., 240.)
                series=series.clone()
                data=move || data.get().unwrap()
                tooltip=tooltip.clone()
            />
        </Transition>
    }
}

#[server(GetPlotData, "/api")]
pub async fn get_data() -> Result<Vec<TimePoint>, ServerFnError> {
    use coingecko::CoinGeckoClient;

    let gecko = CoinGeckoClient::default();
    let history = gecko.coin_market_chart("solana", "eur", 1, true).await;
    let history = match history {
        Ok(history) => history,
        Err(err) => {
            logging::log!("failed to get prices from CoinGecko: {err:?}");
            return Err(ServerFnError::new("failed to load API data"));
        }
    };

    Ok(TimePoint::from_vec(history.prices))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimePoint {
    pub time: f64,
    pub price: f64,
}

#[cfg(feature = "ssr")]
impl TimePoint {
    fn new(time: f64, price: f64) -> Self {
        Self { time, price }
    }

    fn from_vec(data: Vec<Vec<f64>>) -> Vec<Self> {
        data.into_iter()
            .map(|point| Self::new(point[0], point[1]))
            .collect()
    }
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn cast_date(date: f64) -> DateTime<Utc> {
    let nanos = (date % 1_000.0 * 1e6) as u32;
    let secs = (date / 1_000.0) as i64;

    DateTime::<Utc>::from_timestamp(secs, nanos).unwrap_or_default()
}

const GRADIENT: SequentialGradient = (
    Colour::from_rgb(0xec, 0x32, 0x32),
    &[
        Colour::from_rgb(0xef, 0x2a, 0x40),
        Colour::from_rgb(0xf1, 0x22, 0x4d),
        Colour::from_rgb(0xf2, 0x1c, 0x59),
        Colour::from_rgb(0xf2, 0x18, 0x66),
        Colour::from_rgb(0xf1, 0x19, 0x73),
        Colour::from_rgb(0xee, 0x1e, 0x80),
        Colour::from_rgb(0xea, 0x25, 0x8c),
        Colour::from_rgb(0xe5, 0x2d, 0x98),
        Colour::from_rgb(0xde, 0x36, 0xa4),
        Colour::from_rgb(0xd6, 0x3e, 0xaf),
        Colour::from_rgb(0xcd, 0x46, 0xba),
        Colour::from_rgb(0xc2, 0x4e, 0xc4),
        Colour::from_rgb(0xb6, 0x56, 0xcd),
        Colour::from_rgb(0xa9, 0x5d, 0xd5),
        Colour::from_rgb(0x9a, 0x64, 0xdd),
        Colour::from_rgb(0x89, 0x6a, 0xe3),
        Colour::from_rgb(0x76, 0x70, 0xe8),
        Colour::from_rgb(0x60, 0x75, 0xec),
        Colour::from_rgb(0x43, 0x7a, 0xef),
    ],
);
