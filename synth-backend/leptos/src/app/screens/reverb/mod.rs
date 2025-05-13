use crate::SynthEffectState;
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn ReverbDisplay(get_state: impl Fn() -> SynthEffectState) -> impl IntoView {
    let send_effect_val = move |param: &str, set_to: f32| {
        let param = param.to_string();

        spawn_local(async move {
            let send = move || reqwest::get(format!(
                "http://127.0.0.1:3000/synth-state/effect/set/reverb/{param}/{set_to}"
            ));

            for _ in 0..8 {
                if send().await.is_ok() {
                    break;
                }
            }
        });
    };

    let slider_change = move |param: String| {
        move |ev| {
            // send the server a request to set the value of draw_bar
            if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                send_effect_val(&param, val as f32 / 1000.0)
            }
        }
    };
    let params = get_state().params;


    let get_controls: Vec<_> =
        params.into_iter()
            .map(|(param, value)| 
                {
                    view! {
                        <div class="flex-row">
                            <p class="flex-col"> { param.clone() } </p>
                            <p> { "=>" } </p>
                            <p class="flex-col"> { value } </p>
                        </div>
                        <div class="flex-row">
                            <input class="horizontal-slider" type="range" min=0 max=1000 step=0.1 prop:value=move || value * 1000.0 on:input=slider_change(param)/>
                        </div>
                    }
                }).collect();

    // TODO: add on/off button 
    
    view! {
        <h1> "Reverb" </h1>
        <div>
            { get_controls }
        </div>
    }
}
