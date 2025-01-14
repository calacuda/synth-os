use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

mod screens;

pub trait ApiPage: 'static {
    fn loading_screen() -> impl IntoView {
        DefaultLoadingScreen
    }

    fn show(b64_state: ReadSignal<String>) -> impl IntoView;

    fn display(data: Signal<Option<String>>) -> impl IntoView {
        // let loading = || data.get().is_some();
        let (b64_data, set_b64) = signal(String::new());

        view! {
            <Show
                when=move || data.get().is_some()
                fallback=|| Self::loading_screen()
                    // let loading = || page.loading_screen();
            >
                {
                    move || {
                        data.get().map(|dat| set_b64.set(dat));

                        Self::show(b64_data)
                    }
                }
            </Show>
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // TODO: make on sse endpoint for each screen

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/synth-backend.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("synths") view=screens::SynthPage/>
                    <Route path=StaticSegment("synth") view=screens::SynthPage/>
                    <Route path=StaticSegment("effects") view=screens::EffectPage/>
                    <Route path=StaticSegment("effect") view=screens::EffectPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DefaultLoadingScreen() -> impl IntoView {
    view! {
        <div>
            "LOADING..."
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
