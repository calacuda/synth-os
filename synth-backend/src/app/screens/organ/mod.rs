use leptos::{prelude::*, task::spawn_local};
use stepper_synth_backend::pygame_coms::{Knob, SynthEngineState};

#[component]
pub fn OrganDisplay(get_state: impl Fn() -> SynthEngineState + 'static) -> impl IntoView {
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
            let send = move || set_organ_draw_bars((db, set_to));

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

#[leptos::server]
pub async fn set_organ_draw_bars(data: (usize, f32)) -> Result<(), leptos::prelude::ServerFnError> {
    use std::ops::IndexMut;
    use stepper_synth_backend::{
        pygame_coms::SynthEngineType, synth_engines::SynthModule, KnobCtrl,
    };

    let synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>> =
        leptos_actix::extract().await?;

    let (db, set_to) = data;
    let to = set_to;

    if to > 1.0 {
        return Err(ServerFnError::new(
            "can only set draw_bars to numbers between 0.0 and 1.0. no greater, no less.",
        ));
    }

    if db > 8 {
        return Err(ServerFnError::new(
            "there are only 8 drawbars. no greater, no less.",
        ));
    }

    let mut synth = synth.lock().unwrap();
    // let mut seq = synth.midi_sequencer.lock().unwrap();
    let organ = synth.engines.index_mut(SynthEngineType::B3Organ as usize);

    let mut f_s: Vec<Box<dyn FnMut(&mut SynthModule) -> bool>> = vec![
        Box::new(|organ| organ.knob_1(to)),
        Box::new(|organ| organ.knob_2(to)),
        Box::new(|organ| organ.knob_3(to)),
        Box::new(|organ| organ.knob_4(to)),
        Box::new(|organ| organ.knob_5(to)),
        Box::new(|organ| organ.knob_6(to)),
        Box::new(|organ| organ.knob_7(to)),
        Box::new(|organ| organ.knob_8(to)),
    ];
    f_s[db](organ);

    Ok(())
}
