use serde::{Deserialize, Serialize};
use stepper_synth_backend::{effects::EffectType, HashMap};

pub mod app;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynthEffectState {
    pub effect: EffectType,
    pub effect_on: bool,
    pub params: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PowerState {
    #[serde(alias = "on")]
    On,
    #[serde(alias = "off")]
    Off,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
