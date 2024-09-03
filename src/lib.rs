use leptos::prelude::*;
mod api;
mod routes;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
    components::{Outlet, ProtectedParentRoute, Route, Router, Routes},
    path, SsrMode,
};
use routes::{nav::*, stories::*, story::*, users::*};
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
        <ResourceInComponent/>
        <ResourceInIsland/>
        <BlockingResourceInIsland/>
    }
}

// Works as expected
#[component]
fn ResourceInComponent() -> impl IntoView {
    let param = RwSignal::new(123_u8);
    let resource = Resource::new(
        move || param.get(),
        move |param| async move { get_data(param).await },
    );

    view! {
        <Suspense>
            <p>"There are " {move || Suspend::new(async move { resource.await.unwrap_or_default().len() })} " data elements in the compoment"</p>
        </Suspense>
    }
}

// Somehow, this writes "There are 123123 data elements in the island"
#[island]
fn ResourceInIsland() -> impl IntoView {
    let param = RwSignal::new(123_u8);
    let resource = Resource::new(
        move || param.get(),
        move |param| async move { get_data(param).await },
    );

    view! {
        <Suspense>
            <p>"There are " {move || Suspend::new(async move { resource.await.unwrap_or_default().len() })} " data elements in the island"</p>
        </Suspense>
    }
}

// Somehow, this writes "There are 123123 data elements in the island"
#[island]
fn BlockingResourceInIsland() -> impl IntoView {
    let param = RwSignal::new(123_u8);
    let resource = Resource::new_blocking(
        move || param.get(),
        move |param| async move { get_data(param).await },
    );

    view! {
        <Suspense>
            <p>"There are " {move || Suspend::new(async move { resource.await.unwrap_or_default().len() })} " data elements in the island"</p>
        </Suspense>
    }
}

// Server panics when loading the page:
// thread 'tokio-runtime-worker' panicked at /home/vincent/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_graph-0.1.0-beta4/src/owner/stored_value.rs:132:34:
// Dereferenced SendWrapper<T> variable from a thread different to the one it has been created with.
// #[island]
// fn LocalResourceInIsland() -> impl IntoView {
//     let param = RwSignal::new(123_u8);
//     let resource = LocalResource::new(move || async move { get_data(param.get()).await });
//
//     view! {
//         <Suspense>
//             <p>"There are " {move || Suspend::new(async move { resource.await.unwrap_or_default().len() })} " data elements in the island"</p>
//         </Suspense>
//     }
// }

// Panics in the client (unwrap on a none value) + Runtime unreachable
// #[island]
// fn ArcLocalResourceInIsland() -> impl IntoView {
//     let param = RwSignal::new(123_u8);
//     let resource = ArcLocalResource::new(move || async move { get_data(param.get()).await });

//     let suspend = move || {
//         let resource = resource.clone();
//         Suspend::new(async move { resource.await.unwrap_or_default().len() })
//     };

//     view! {
//         <Suspense>
//             <p>"There are " {suspend} " data elements in the island"</p>
//         </Suspense>
//     }
// }

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
