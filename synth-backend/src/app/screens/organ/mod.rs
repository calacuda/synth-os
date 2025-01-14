use leptos::{prelude::*, task::spawn_local};
use stepper_synth_backend::pygame_coms::{Knob, SynthEngineState};

#[component]
pub fn OrganDisplay(get_state: impl Fn() -> SynthEngineState) -> impl IntoView {
    let db_vals = [
        Knob::One,
        Knob::Two,
        Knob::Three,
        Knob::Four,
        Knob::Five,
        Knob::Six,
        Knob::Seven,
        Knob::Eight,
    ];
    let get_param = move |knob| get_state().knob_params.get(&knob).unwrap().clone();
    let knobs: Vec<_> = db_vals
        .into_iter()
        .map(|knob| signal((get_param(knob) * 8.0) as usize))
        .collect();

    let send_db = move |db: usize, set_to: f32| {
        spawn_local(async move {
            let send = move || {
                reqwest::get(format!(
                    "http://127.0.0.1:3000/synth-state/engine/set/organ/draw-bar/{db}/{set_to}"
                ))
            };

            for _ in 0..8 {
                if send().await.is_ok() {
                    break;
                }
            }
        });
    };

    let draw_bar_change = move |i: usize| {
        move |ev| {
            // send the server a request to set the value of draw_bar
            if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                send_db(i, val as f32 / 8.0)
            }
        }
    };

    let draw_bars: Vec<_> = knobs.into_iter().enumerate().map(move |(i, knob)| {
                    view! {
                        // <div>
                            <p> {format!("{}", i + 1)} </p>
                            <div class="flex-col">
                                <input class="vert-slider" type="range" min=0 max=8 step=1 prop:value=move || knob.0.get() on:input=draw_bar_change(i)/>
                                <p> { move || knob.0.get() } </p>
                            </div>
                        // </div>
                    }
                }).collect();

    view! {
        <h1> "Organ" </h1>
        <div class="flex-row">
            { draw_bars }
        </div>
    }
}
