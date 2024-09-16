use leptos::prelude::*;
mod api;
mod dummy;
mod routes;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
    components::{Outlet, ProtectedParentRoute, Route, Router, Routes},
    path, SsrMode,
};
use routes::{nav::*, stories::*, story::*, users::*};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
pub mod fallback;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[server]
pub async fn check_protection() -> Result<bool, ServerFnError> {
    Ok(true)
}

#[server]
pub async fn get_data(max: u8) -> Result<Vec<u8>, ServerFnError> {
    Ok((0..max).collect::<Vec<_>>())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos implementation of a HackerNews demo."/>
        <ContextProvider>
            <IslandRoutes/>
        </ContextProvider>
    }
}

#[island]
fn ContextProvider(children: Children) -> impl IntoView {
    let permission = Resource::new(
        || (),
        |()| async move { check_protection().await.ok().unwrap_or_default() },
    );
    provide_context(permission);

    view! { {children()} }
}

#[component]
fn IslandRoutes() -> impl IntoView {
    let permission = expect_context::<Resource<bool>>();
    view! {
        <Router>
            <Nav />
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("users/:id") view=User/>
                    <Route path=path!("stories/:id") view=Story/>
                    <Route path=path!("stories") view=Stories/>
                    <ProtectedParentRoute
                        path=path!("testing")
                        condition=move || permission.get()
                        ssr=SsrMode::Async
                        redirect_path=|| "/"
                        view=move || {
                            view! {
                                <TestingComponent />
                                <Outlet />
                            }
                        }
                    >

                        <Route path=path!("") view=|| () />
                    </ProtectedParentRoute>
                    // TODO allow optional params without duplication
                    <Route path=path!("") view=Stories/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn TestingComponent() -> impl IntoView {
    view! {
        <Slider initial_position=SliderPosition::Off style=String::new()>
            <DisplaySlider />
        </Slider>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SliderPosition {
    On,
    Off,
}

#[island]
pub fn Slider(
    initial_position: SliderPosition,
    style: String,
    children: Children,
) -> impl IntoView {
    let pos = RwSignal::new(initial_position);
    provide_context(pos);

    let change = move |_ev| match pos.get() {
        SliderPosition::On => *pos.write() = SliderPosition::Off,
        SliderPosition::Off => *pos.write() = SliderPosition::On,
    };

    view! {
        <div style=style>
            <label class="switch">
                <input type="checkbox" on:change=change />
                <span class="slider round"></span>
            </label>
        </div>
        {children()}
    }
}
#[island]
pub fn DisplaySlider() -> impl IntoView {
    let slider = expect_context::<RwSignal<SliderPosition>>();
    let pos = move || {
        if slider.read() == SliderPosition::On {
            "On"
        } else {
            "Off"
        }
    };
    view! { <p>"Slider is in position " {pos}</p> }
}
