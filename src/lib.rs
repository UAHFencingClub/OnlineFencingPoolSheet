use fencing_sport_lib::{fencer::SimpleFencer, pools::PoolResults};
use leptos::*;

// Modules
mod components;

use components::{
    fencer_list::{FencerList, FencerListError},
    pool_sheet::PoolSheet,
    result_sheet::PoolResultTable,
};
use leptos_dom::Text;

#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Err(FencerListError::NoFencers));
    let (results, set_results) = create_signal(None::<PoolResults<SimpleFencer>>);

    view! {
        <FencerList submit_fencers=set_competitors/>
        {move || {
            match competiors.get() {
                Ok(fencers) => {
                    view! {
                        <PoolSheet
                            fencers=fencers
                            on_complete=move |results| { set_results.set(Some(results)) }
                        />
                    }
                }
                Err(err) => View::Text(Text::new(Oco::Owned(format!("{err:?}")))),
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
