use fencing_sport_lib::fencer::SimpleFencer;
use leptos::*;

// Modules
mod components;

use components::{fencer_list::FencerList, pool_sheet::PoolSheet};

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
