use leptos::{prelude::*, task::spawn_local};
use stepper_synth_backend::pygame_coms::{Knob, SynthEngineState};

#[component]
pub fn WurlitzerDisplay(get_state: impl Fn() -> SynthEngineState) -> impl IntoView {
    let get_param = move |knob| get_state().knob_params.get(&knob).unwrap().clone();
    let knob = signal(get_param(Knob::One));

    let send_trem = move |set_to: f32| {
        spawn_local(async move {
            let send = || {
                reqwest::get(format!(
                    "http://127.0.0.1:3000/synth-state/engine/set/wurlitzer/trem/{set_to}"
                ))
            };

            for _ in 0..8 {
                if send().await.is_ok() {
                    break;
                }
            }
        });
    };

    view! {
        <h1> "Wurlitzer" </h1>
        <div class="flex-row">
            <input class="vert-slider" type="range" min=0 max=1000 step=1 prop:value=move || knob.0.get() * 1000.0 on:input=move |ev| {
                    // send the server a request to set the value of draw_bar
                    if let Ok(val) = event_target_value(&ev).parse::<usize>() {
            send_trem(val as f32 / 1000.0)
                    }
                }
            />
        </div>
    }
}
