use leptos::prelude::*;
use leptos_router::components::A;

#[island]
pub fn Nav() -> impl IntoView {
    let permission = expect_context::<Resource<bool>>();
    view! {
        <header class="header">
            <nav class="inner">
                <a href="/home">
                    <strong>"HN"</strong>
                </a>
                <a href="/new">
                    <strong>"New"</strong>
                </a>
                <a href="/show">
                    <strong>"Show"</strong>
                </a>
                <a href="/ask">
                    <strong>"Ask"</strong>
                </a>
                <a href="/job">
                    <strong>"Jobs"</strong>
                </a>
                <Suspense>
                    <Show when=move || permission.get().unwrap_or(false) fallback=|| view! { <a href="#">"loading"</a> }>
                    <InnerNav/>
                    </Show>
                </Suspense>
                <a class="github" href="http://github.com/leptos-rs/leptos" target="_blank" rel="noreferrer">
                    "Built with Leptos"
                </a>
            </nav>
        </header>
    }
}

#[island]
fn InnerNav() -> impl IntoView {
    let value = RwSignal::new(false);

    view! {
        <button on:click=move |_ev| value.update(|val| *val = !*val)>{value}</button>
    }
}
