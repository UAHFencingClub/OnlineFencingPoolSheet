use fencing_sport_lib::{fencer::SimpleFencer, pools::PoolResults};
use leptos::*;

// Modules
mod components;

use components::{fencer_list::FencerList, pool_sheet::PoolSheet, result_sheet::PoolResultTable};

#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Vec::<SimpleFencer>::new());
    let (results, set_results) = create_signal(None::<PoolResults<SimpleFencer>>);

    view! {
        <FencerList submit_fencers=set_competitors/>
        {move || {
            view! {
                <PoolSheet
                    fencers=competiors.get()
                    on_complete=move |results| { set_results.set(Some(results)) }
                />
            }
        }}

        {move || {
            match results.get() {
                Some(results) => view! { <PoolResultTable pool_results=results/> }.into_view(),
                None => view! { <p>"No Results Yet"</p> }.into_view(),
            }
        }}
    }
}
