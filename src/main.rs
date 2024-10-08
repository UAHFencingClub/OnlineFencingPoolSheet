// use fencing_pool_sheet_leptos::App;
use leptos::*;

use fencing_pool_sheet_leptos::components::{
    fencer_list::{FencerList, FencerListError},
    pool_sheet::PoolSheet,
    result_sheet::PoolResultTable,
};
use leptos::wasm_bindgen::JsCast;

use fencing_sport_lib::{
    fencer::SimpleFencer,
    pools::{PoolResults, PoolSheetError},
};

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let (competiors, set_competitors) = create_signal(Err(FencerListError::NoFencers));
    let (results, set_results) =
        create_signal(Result::<PoolResults<SimpleFencer>, PoolSheetError>::Err(
            PoolSheetError::PoolNotComplete(Vec::<usize>::new()),
        ));

    let doc = leptos::document();
    let fencer_main_div = doc
        .get_element_by_id("fencer-list-main-div")
        .expect("Element should exist in the index.html");

    let tab_content_div = doc
        .get_element_by_id("tab-content-div")
        .expect("Element should exist in index.html");

    mount_to(fencer_main_div.clone().unchecked_into(), move || {
        view! { <FencerList submit_fencers=set_competitors/> }
    });

    mount_to(tab_content_div.clone().unchecked_into(), move || {
        view! {
            <div class="row mb-2">
                {move || {
                    match competiors.get() {
                        Ok(fencers) => {
                            view! {
                                <PoolSheet
                                    fencers=fencers
                                    on_complete=move |results| { set_results.set(results) }
                                />
                            }
                                .into_view()
                        }
                        Err(err) => view! { <p>{format!("Error: {err}")}</p> }.into_view(),
                    }
                }}

            </div>
            <div class="row">
                {move || {
                    match results.get() {
                        Ok(results) => view! { <PoolResultTable pool_results=results/> }.into_view(),
                        Err(err) => view! { <p>{format!("Error: {err}")}</p> }.into_view(),
                    }
                }}

            </div>
        }
    })
}
