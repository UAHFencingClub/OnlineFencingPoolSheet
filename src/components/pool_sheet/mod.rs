use std::str::FromStr;
use std::{fmt::format, rc::Rc};

use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    fencer::{Fencer, SimpleFencer},
    pools::{bout_creation::SimpleBoutsCreator, PoolSheet},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde_json;

#[component]
pub fn PoolSheet(fencers: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div id="pool-sheet">
            {move || {
                let fencers: Vec<SimpleFencer> = fencers
                    .get()
                    .into_iter()
                    .map(|fencer_str| { SimpleFencer::new(fencer_str) })
                    .collect();
                let (poolsheet_sig, set_poolsheet_sig) = create_signal(
                    PoolSheet::new(fencers, &SimpleBoutsCreator).ok(),
                );
                poolsheet_sig
                    .with(|poolsheet_option| {
                        match poolsheet_option {
                            Some(poolsheet) => view! { "Hello Sheet" },
                            None => view! { "Goodbye Sheet" },
                        }
                    })
            }}

        </div>
    }
}

#[component]
pub fn PoolSheetTable() -> impl IntoView {}

#[component]
fn BoutList() -> impl IntoView {}

#[component]
pub fn BoutScoreTableCell(
    main_fencer: SimpleFencer,
    secondary_fencer: SimpleFencer,
) -> impl IntoView {
}

#[component]
pub fn PoolTableHeader() -> impl IntoView {}

#[component]
pub fn PoolFencerTableRow(children: Children) -> impl IntoView {}
