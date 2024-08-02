use fencing_sport_lib::{fencer::SimpleFencer, pools::PoolResults};
use leptos::*;

// Modules
mod components;

use components::{fencer_list::FencerList, pool_sheet::PoolSheet, result_sheet::PoolResultTable};

#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Vec::<String>::new());
    let (results, set_results) = create_signal(None::<PoolResults<SimpleFencer>>);

    view! {
        <FencerList submit_fencers=set_competitors/>
        {move || {
            let fencers: Vec<SimpleFencer> = competiors
                .get()
                .into_iter()
                .map(|fencer_str| { SimpleFencer::new(fencer_str) })
                .collect();
            view! {
                <PoolSheet
                    fencers=fencers
                    on_complete=move |results| { set_results.set(Some(results)) }
                />
            }
        }}

        {move || {
            match results.get() {
                Some(results) => view! { <PoolResultTable pool_results=results/> },
                None => View::Text(view! { "Goodby Results" }),
            }
        }}
    }
}
