use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{components::*, StaticSegment, WildcardSegment};
use screens::channel_editor::ChannelEditor;

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
        <Title text="Net Synth"/>

        // content for this welcome page
        <Router>
            <main class="flex flex-row w-dvw h-dvh">
                <nav class="w-[10%] h-full">
                    // TODO: add side bar component here
                    <SideBar/>
                </nav>
                <div class="w-[90%] h-full">
                    <Routes fallback=move || "Not found.">
                        // <Route path=StaticSegment("") view=HomePage/>
                        // <Route path=StaticSegment("midi-stepper") view=MidiStepper/>
                        // <Route path=StaticSegment("midi-seq") view=MidiSequencer/>
                        <Route path=StaticSegment("edit") view=ChannelEditor/>
                        // <Route path=path!("/channel/:channel") view=Channel/>
                        // <Route path=StaticSegment("settings") view=Settings/>
                        <Route path=WildcardSegment("any") view=HomePage/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

#[component]
fn SideBar() -> impl IntoView {
    view! {
        // <div class="justify-center">
        // <ol class="text-center felx-col items-center h-[50%] w-full ">
        <aside class="h-[50%] w-full flex flex-col text-center align-text-middle justify-stretch">
            // <li class="w-full"><a class="w-full" href="/midi-stepper">Stepper</a></li>
            // <li class="w-full"><a class="w-full" href="/midi-seq">Sequencer</a></li>
            // <li class="w-full"><a class="w-full" href="/edit">Edit Channels</a></li>
            // <li class="w-full"><a class="w-full" href="/channel/A">A</a></li>
            // <li class="w-full"><a class="w-full" href="/channel/B">B</a></li>
            // <li class="w-full"><a class="w-full" href="/channel/C">C</a></li>
            // <li class="w-full"><a class="w-full" href="/channel/D">D</a></li>
            // <li class="w-full"><a class="w-full" href="/settings">Settings</a></li>
            <a class="w-full h-[12.5%] align-text-middle" href="/midi-stepper">Stepper</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/midi-seq">Sequencer</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/edit">Edit Channels</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/channel/A">A</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/channel/B">B</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/channel/C">C</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/channel/D">D</a>
            <a class="w-full h-[12.5%] align-text-middle" href="/settings">Settings</a>
        // </ol>
        </aside>
        // </div>
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

// /// 404 - Not Found
// #[component]
// fn NotFound() -> impl IntoView {
//     // set an HTTP status code 404
//     // this is feature gated because it can only be done during
//     // initial server-side rendering
//     // if you navigate to the 404 page subsequently, the status
//     // code will not be set because there is not a new HTTP request
//     // to the server
//     #[cfg(feature = "ssr")]
//     {
//         // this can be done inline because it's synchronous
//         // if it were async, we'd use a server function
//         let resp = expect_context::<leptos_actix::ResponseOptions>();
//         resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
//     }
//
//     view! {
//         <h1>
//             "Not Found"
//         </h1>
//     }
// }
