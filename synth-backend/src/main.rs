#![feature(impl_trait_in_bindings)]

#[cfg(feature = "ssr")]
mod consts {
    // pub const API_DIR: &str = "/tmp/synth/";
    // pub const SEQ_SOCKET: &str = "sequencer.sock";
    pub const API_SOCKET: &str = "/tmp/synth/backend.sock";
}

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
    use std::{fs::create_dir, path::PathBuf};
    use stepper_synth_backend::pygame_coms::StepperSynth;
    use synth_backend::app::*;
    use tokio::sync::Mutex;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let socket_parent = PathBuf::from(consts::API_SOCKET);
    let socket_parent = socket_parent.parent().expect("this exists");

    if !socket_parent.exists() {
        _ = create_dir(socket_parent);
    }

    let synth = web::Data::new(Mutex::new(StepperSynth::new()));

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        // println!("listening on http://{}", &addr);
        // println!("listening on {}", &consts::API_SOCKET);

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
            .service(set_synth_engine)
            .service(set_organ_draw_bars)
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
                                <AutoReload options=leptos_options.clone() />
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
    synth: actix_web::web::Data<
        tokio::sync::Mutex<stepper_synth_backend::pygame_coms::StepperSynth>,
    >,
    engine: actix_web::web::Path<stepper_synth_backend::pygame_coms::SynthEngineType>,
) -> impl actix_web::Responder {
    synth.lock().await.set_engine(engine.into_inner());

    String::new()
}

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine/set/organ/draw-bar/{db}/{set_to}")]
pub async fn set_organ_draw_bars(
    synth: actix_web::web::Data<
        tokio::sync::Mutex<stepper_synth_backend::pygame_coms::StepperSynth>,
    >,
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

    let synth = synth.lock().await;
    let mut seq = synth.midi_sequencer.lock().unwrap();
    let organ = seq
        .synth
        .engines
        .index_mut(SynthEngineType::B3Organ as usize);

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

#[cfg(feature = "ssr")]
#[actix_web::get("/synth-state/engine")]
pub async fn synth_engine_state(
    // req: actix_web::HttpRequest,
    // stream: actix_web::web::Payload,
    synth: actix_web::web::Data<
        tokio::sync::Mutex<stepper_synth_backend::pygame_coms::StepperSynth>,
    >,
) -> impl actix_web::Responder {
    use actix_web_lab::sse;
    use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
    use std::time::Duration;

    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    let mut send_state = {
        let synth = synth.clone();
        let mut last_state = Vec::new();

        async move || {
            let state = {
                let state = synth.lock().await.get_engine_state();
                state
            };

            if let Ok(msg) = bincode::serialize(&state) {
                if last_state == msg {
                    return true;
                }
                last_state = msg.clone();

                let msg = sse::Data::new(BASE64_STANDARD_NO_PAD.encode(&msg));
                // println!("B2");
                // let res = sender.try_send(msg.into());
                // println!("A2");

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
            // let time = time::OffsetDateTime::now_utc();
            // let msg = sse::Data::new(time.format(&Rfc3339).unwrap()).event("timestamp");
            //
            // if sender.send(msg.into()).await.is_err() {
            //     tracing::warn!("client disconnected; could not send SSE message");
            //     break;
            // }
            //
            // sleep(Duration::from_secs(10)).await;
            actix_web::rt::time::sleep(Duration::from_millis(1000)).await;
            // let res = synth.lock().await;

            // if res.is_err() {
            //     println!("{res:?}");
            // }

            // if res.updated() {
            let keep_going = send_state().await;

            if !keep_going {
                println!("stopping sse");
                break;
            }
            // } else {
            // println!("nope")
            // }
        }

        println!("done");
    });

    sse::Sse::from_infallible_receiver(receiver) // .with_keep_alive(Duration::from_secs(1))
}

// #[cfg(feature = "ssr")]
// pub async fn synth_state(
//     req: actix_web::HttpRequest,
//     stream: actix_web::web::Payload,
//     synth: actix_web::web::Data<std::sync::Mutex<stepper_synth_backend::pygame_coms::StepperSynth>>,
// ) -> impl actix_web::Responder {
//     use leptos::prelude::*;
//     use std::time::Duration;
//     use stepper_synth_backend::pygame_coms::StepperSynthState;
//     // use actix_web_lab::sse;
//     // use futures::stream;
//     use leptos_sse::ServerSentEvents;
//     // use tokio_stream::StreamExt as _;
//
//     // let (res, session, _msg_stream) = actix_ws::handle(&req, stream).unwrap();
//     // let mut count = ServerSignal::<Count>::new("counter", session).unwrap();
//     // let mut state = ServerSignal::<Option<StepperSynthState>>::new("synth-state", session).unwrap();
//     //
//     // actix_web::rt::spawn(async move {
//     //     loop {
//     //         actix_web::rt::time::sleep(Duration::from_millis(100)).await;
//     //         // let result = count.with(|count| count.value += 1).await;
//     //         let result = state.set(Some(synth.lock().unwrap().get_state()));
//     //
//     //         if result.is_err() {
//     //             break;
//     //         }
//     //     }
//     // });
//
//     // res
// }

// #[cfg(not(any(feature = "ssr", feature = "csr")))]
// pub fn main() {
//     // no client-side main function
//     // unless we want this to work with e.g., Trunk for pure client-side testing
//     // see lib.rs for hydration function instead
//     // see optional feature `csr` instead
// }

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
