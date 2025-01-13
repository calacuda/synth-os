use leptos::{prelude::*, task::spawn_local};
use stepper_synth_backend::pygame_coms::{Knob, SynthEngineState};

#[component]
pub fn SubSynthDisplay(get_state: impl Fn() -> SynthEngineState) -> impl IntoView {
    // TODO: add getter/setter for GuiParams

    let db_vals = [
        Knob::One,
        Knob::Two,
        Knob::Three,
        Knob::Four,
        Knob::Five,
        Knob::Six,
        // Knob::Seven,
        // Knob::Eight,
    ];
    let get_param = move |knob| get_state().knob_params.get(&knob).unwrap().clone();
    let knobs: Vec<_> = db_vals
        .into_iter()
        .map(|knob| signal((get_param(knob) * 8.0) as usize))
        .collect();

    let send_trem = move |set_to: f32| {
        spawn_local(async move {
            // _ = reqwest::get(format!(
            //     "http://127.0.0.1:3000/synth-state/engine/set/wurlitzer/trem/{set_to}"
            // ))
            // .await;
        });
    };

    view! {
        <h1> "Subtrctive Synth" </h1>
        <div class="flex-row">
            // <input class="vert-slider" type="range" min=0 max=1000 step=1 prop:value=move || knob.0.get() * 1000.0 on:input=move |ev| {
            //         // send the server a request to set the value of draw_bar
            //         if let Ok(val) = event_target_value(&ev).parse::<usize>() {
            //             send_trem(val as f32 / 1000.0)
            //         }
            //     }
            // />
        </div>
    }
}
