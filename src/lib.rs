use fencing_sport_lib::{
    fencer::SimpleFencer,
    pools::{PoolResults, PoolSheetError},
};
use leptos::*;

// Modules
mod components;

use components::{
    fencer_list::{FencerList, FencerListError},
    pool_sheet::PoolSheet,
    result_sheet::PoolResultTable,
};

#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Err(FencerListError::NoFencers));
    let (results, set_results) = create_signal(
        Result::<PoolResults<SimpleFencer>, PoolSheetError>::Err(PoolSheetError::PoolNotComplete),
    );

    view! {
        <FencerList submit_fencers=set_competitors/>
        {move || {
            match competiors.get() {
                Ok(fencers) => {
                    view! {
                        <PoolSheet
                            fencers=fencers
                            on_complete=move |results| { set_results.set(results) }
                        />
                    }
                }
                Err(err) => view! { <p>{format!("{err:?}")}</p> }.into_view(),
            }
        }}

        {move || {
            match results.get() {
                Ok(results) => view! { <PoolResultTable pool_results=results/> }.into_view(),
                Err(err) => view! { <p>{format!("{err:?}")}</p> }.into_view(),
            }
        }}
    }
}
