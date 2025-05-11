#![feature(impl_trait_in_bindings)]
use leptos::prelude::{server, ServerFnError};
use leptos_actix::extract;

#[cfg(feature = "ssr")]
mod consts {
    // pub const API_DIR: &str = "/tmp/synth/";
    // pub const SEQ_SOCKET: &str = "sequencer.sock";
    pub const API_SOCKET: &str = "/tmp/synth/backend.sock";
}

#[cfg(feature = "ssr")]
mod synth_helpers;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::middleware::Logger;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use log::*;
    use std::{
        fs::create_dir,
        path::PathBuf,
        sync::{atomic::AtomicBool, Arc, Mutex},
        usize,
    };
    use stepper_synth_backend::SampleGen;
    use stepper_synth_backend::{sequencer::SequencerIntake, synth_engines::Synth, SAMPLE_RATE};
    use synth_backend::app::*;
    use synth_helpers::run_midi;
    use tinyaudio::{run_output_device, OutputDeviceParameters};
    use tokio::task::spawn;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let socket_parent = PathBuf::from(consts::API_SOCKET);
    let socket_parent = socket_parent.parent().expect("this exists");

    if !socket_parent.exists() {
        _ = create_dir(socket_parent);
    }

    // TODO: enable midi sequencer
    let seq = web::Data::new(Mutex::new(SequencerIntake::new()));
    let synth = web::Data::new(std::sync::Mutex::new(Synth::new()));
    // synth.lock().unwrap().set_engine(SynthEngineType::SubSynth);
    let exit: Arc<AtomicBool> = Arc::new(false.into());

    let _jh = {
        let seq = seq.clone();
        let synth = synth.clone();

        spawn(async move { run_midi(seq, synth, exit).await });
    };
    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: SAMPLE_RATE as usize,
        // channel_sample_count: 2048,
        channel_sample_count: 1024,
    };
    let device = run_output_device(params, {
        let synth = synth.clone();

        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                let value = synth.lock().unwrap().get_sample();

                for sample in samples {
                    *sample = value;
                }
            }
        }
    });

    if let Err(e) = device {
        error!("starting audio playback caused error: {e}");
    }

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            .wrap(Logger::default())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            // .service(synth_state)
            .service(synth_engine_state)
            .service(synth_effect_state)
            .service(set_synth_engine)
            .service(set_organ_draw_bars)
            .service(set_wurli_trem)
            .service(set_reverb_params)
            .service(set_effect)
            .service(set_effect_power)
            // .route("/synth-state", web::get().to(synth_state))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();

                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone()/>
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(synth.clone())
            .app_data(seq.clone())
        //.wrap(middleware::Compress::default())
    })
        .workers(6)
    .bind(&addr)?
    .bind_uds(&consts::API_SOCKET)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine/set/{engine}")]
pub async fn set_synth_engine(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    engine: actix_web::web::Path<stepper_synth_backend::pygame_coms::SynthEngineType>,
) -> impl actix_web::Responder {
    synth.lock().unwrap().set_engine(engine.into_inner());

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine/set/organ/draw-bar/{db}/{set_to}")]
pub async fn set_organ_draw_bars(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    data: actix_web::web::Path<(usize, f32)>,
) -> impl actix_web::Responder {
    use std::ops::IndexMut;
    use stepper_synth_backend::{
        pygame_coms::SynthEngineType, synth_engines::SynthModule, KnobCtrl,
    };

    let (db, set_to) = data.into_inner();
    let to = set_to;

    if to > 1.0 {
        return "can only set draw_bars to numbers between 0.0 and 1.0. no greater, no less."
            .to_string();
    }

    if db > 8 {
        return "there are only 8 drawbars. no greater, no less.".to_string();
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

    String::new()
}

// #[leptos::server]
// pub async fn set_organ_draw_bars_sfn(
//     data: (usize, f32),
// ) -> Result<(), leptos::prelude::ServerFnError> {
//     use std::ops::IndexMut;
//     use stepper_synth_backend::{
//         pygame_coms::SynthEngineType, synth_engines::SynthModule, KnobCtrl,
//     };
//
//     let synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>> =
//         extract().await?;
//     // let data: actix_web::web::Path<(usize, f32)> = extract().await?;
//
//     let (db, set_to) = data;
//     let to = set_to;
//
//     if to > 1.0 {
//         return Err(ServerFnError::new(
//             "can only set draw_bars to numbers between 0.0 and 1.0. no greater, no less."
//                 .to_string(),
//         ));
//     }
//
//     if db > 8 {
//         return Err(ServerFnError::new(
//             "there are only 8 drawbars. no greater, no less.",
//         ));
//     }
//
//     let mut synth = synth.lock().unwrap();
//     // let mut seq = synth.midi_sequencer.lock().unwrap();
//     let organ = synth.engines.index_mut(SynthEngineType::B3Organ as usize);
//
//     let mut f_s: Vec<Box<dyn FnMut(&mut SynthModule) -> bool>> = vec![
//         Box::new(|organ| organ.knob_1(to)),
//         Box::new(|organ| organ.knob_2(to)),
//         Box::new(|organ| organ.knob_3(to)),
//         Box::new(|organ| organ.knob_4(to)),
//         Box::new(|organ| organ.knob_5(to)),
//         Box::new(|organ| organ.knob_6(to)),
//         Box::new(|organ| organ.knob_7(to)),
//         Box::new(|organ| organ.knob_8(to)),
//     ];
//     f_s[db](organ);
//
//     Ok(())
// }

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine/set/wurlitzer/trem/{set_to}")]
pub async fn set_wurli_trem(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    data: actix_web::web::Path<f32>,
) -> impl actix_web::Responder {
    use std::ops::IndexMut;
    use stepper_synth_backend::{pygame_coms::SynthEngineType, KnobCtrl};

    let set_to = data.into_inner();

    if set_to > 1.0 {
        return "can only set draw_bars to numbers between 0.0 and 1.0. no greater, no less."
            .to_string();
    }

    let mut synth = synth.lock().unwrap();
    let wurli = synth.engines.index_mut(SynthEngineType::Wurlitzer as usize);

    wurli.knob_1(set_to);

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/effect/set/{effect}")]
pub async fn set_effect(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    data: actix_web::web::Path<stepper_synth_backend::effects::EffectType>,
) -> impl actix_web::Responder {
    let effect = data.into_inner();
    let mut synth = synth.lock().unwrap();

    synth.effect_type = effect;

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/effect/{power}")]
pub async fn set_effect_power(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    data: actix_web::web::Path<synth_backend::PowerState>,
) -> impl actix_web::Responder {
    use synth_backend::PowerState;

    let power = data.into_inner();
    let mut synth = synth.lock().unwrap();

    synth.effect_power = power == PowerState::On;

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/effect/set/reverb/{param}/{set_to}")]
pub async fn set_reverb_params(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
    data: actix_web::web::Path<(String, f32)>,
) -> impl actix_web::Responder {
    use std::ops::IndexMut;
    use stepper_synth_backend::effects::{Effect, EffectType};

    let (param, set_to) = data.into_inner();

    if set_to > 1.0 {
        return "number must be between 0.0 and 1.0. no greater, no less.".to_string();
    }

    let mut synth = synth.lock().unwrap();
    let reverb = synth.effects.index_mut(EffectType::Reverb as usize);

    reverb.set_param(&param, set_to);

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine")]
pub async fn synth_engine_state(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
) -> impl actix_web::Responder {
    use actix_web_lab::sse;
    use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
    use std::time::Duration;
    use stepper_synth_backend::{pygame_coms::SynthEngineState, synth_engines::SynthEngine};

    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    let mut send_state = {
        let synth = synth.clone();
        let mut last_state = Vec::new();

        async move || {
            let state = {
                // let state = synth.lock().await.get_engine_state();
                // state
                let mut synth = synth.lock().unwrap();

                SynthEngineState {
                    engine: synth.engine_type,
                    effect: synth.effect_type,
                    effect_on: synth.effect_power,
                    knob_params: synth.get_engine().get_params(),
                    gui_params: synth.get_engine().get_gui_params(),
                }
            };

            if let Ok(msg) = bincode::serialize(&state) {
                if last_state == msg {
                    return true;
                }
                last_state = msg.clone();

                let msg = sse::Data::new(BASE64_STANDARD_NO_PAD.encode(&msg));

                if sender.try_send(msg.into()).is_err() {
                    println!("client disconnected; could not send SSE message");
                    false
                } else {
                    true
                }
            } else {
                true
            }
        }
    };

    send_state().await;

    actix_web::rt::spawn(async move {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(1000)).await;

            let keep_going = send_state().await;

            if !keep_going {
                println!("stopping sse");
                break;
            }
        }

        println!("done");
    });

    sse::Sse::from_infallible_receiver(receiver) // .with_keep_alive(Duration::from_secs(1))
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/effect")]
pub async fn synth_effect_state(
    synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::synth_engines::Synth>>,
) -> impl actix_web::Responder {
    use actix_web_lab::sse;
    use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
    use std::time::Duration;
    use stepper_synth_backend::effects::Effect;
    use synth_backend::SynthEffectState;

    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    let mut send_state = {
        let synth = synth.clone();
        let mut last_state = Vec::new();

        async move || {
            let state = {
                let mut synth = synth.lock().unwrap();

                SynthEffectState {
                    effect: synth.effect_type,
                    effect_on: synth.effect_power,
                    params: synth.get_effect().get_params(),
                }
            };

            if let Ok(msg) = bincode::serialize(&state) {
                if last_state == msg {
                    return true;
                }
                last_state = msg.clone();

                let msg = sse::Data::new(BASE64_STANDARD_NO_PAD.encode(&msg));

                if sender.try_send(msg.into()).is_err() {
                    println!("client disconnected; could not send SSE message");
                    false
                } else {
                    true
                }
            } else {
                true
            }
        }
    };

    send_state().await;

    actix_web::rt::spawn(async move {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(1000)).await;

            let keep_going = send_state().await;

            if !keep_going {
                println!("stopping sse");
                break;
            }
        }

        println!("done");
    });

    sse::Sse::from_infallible_receiver(receiver) // .with_keep_alive(Duration::from_secs(1))
}

// #[cfg(not(any(feature = "ssr", feature = "csr")))]
// pub fn main() {
//     // no client-side main function
//     // unless we want this to work with e.g., Trunk for pure client-side testing
//     // see lib.rs for hydration function instead
//     // see optional feature `csr` instead
// }
//
// #[cfg(all(not(feature = "ssr"), feature = "csr"))]
// pub fn main() {
//     // a client-side main function is required for using `trunk serve`
//     // prefer using `cargo leptos serve` instead
//     // to run: `trunk serve --open --features csr`
//     use synth_backend::app::*;
//
//     console_error_panic_hook::set_once();
//
//     leptos::mount_to_body(App);
// }
