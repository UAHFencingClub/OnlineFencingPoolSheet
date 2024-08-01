use std::fmt::format;
use std::str::FromStr;

use fencing_sport_lib::{
    bout::FencerScore,
    fencer::{Fencer, SimpleFencer},
    pools::PoolSheet,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::{info, log};
use serde_json;

// Modules
mod components;

use components::{fencer_list::FencerList, pool_sheet::PoolSheet};

// use console_log::info;

// // Top-Level pages
// use crate::pages::home::Home;
// use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Vec::<String>::new());
    view! {
        <FencerList submit_fencers=set_competitors/>
        {move || {
            let fencers: Vec<SimpleFencer> = competiors
                .get()
                .into_iter()
                .map(|fencer_str| { SimpleFencer::new(fencer_str) })
                .collect();
            view! { <PoolSheet fencers=fencers/> }
        }}
    }
}
