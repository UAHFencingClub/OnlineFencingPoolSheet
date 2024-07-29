use std::str::FromStr;
use std::{fmt::format, rc::Rc};

use fencing_sport_lib::bout::Bout;
use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    fencer::{Fencer, SimpleFencer},
    pools::{bout_creation::SimpleBoutsCreator, PoolBoutIter, PoolSheet, PoolSheetBout},
};
use itertools::Itertools;
use leptos::*;
use leptos_dom::Text;
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
                            Some(poolsheet) => {
                                view! {
                                    <PoolSheetTable poolsheet=poolsheet/>
                                    <BoutList bouts=poolsheet.iter_bouts()/>
                                }
                            }
                            None => Fragment::new(vec![View::Text(view! { "Goodbyte Poolsheet" })]),
                        }
                    })
            }}

        </div>
    }
}

#[component]
pub fn PoolSheetTable<'a>(poolsheet: &'a PoolSheet<SimpleFencer>) -> impl IntoView {
    let fencers = poolsheet.get_fencers();
    view! {
        <table>
            <PoolTableHeader fencers=&fencers/>
            {fencers
                .iter()
                .map(|fencer| view! { <PoolTableRow main_fencer=fencer fencers=&fencers/> })
                .collect::<Vec<_>>()}
        </table>
    }
}

#[component]
fn BoutList<'a>(bouts: PoolBoutIter<'a, SimpleFencer>) -> impl IntoView {
    view! {
        <ol>
            {bouts
                .into_iter()
                .map(|(vs, bout)| {
                    view! { <li>{vs.0.get_fullname()} vs {vs.1.get_fullname()}</li> }
                })
                .collect::<Vec<_>>()}
        </ol>
    }
}

#[component]
pub fn BoutScoreTableCell<'a>(
    main_fencer: &'a SimpleFencer,
    secondary_fencer: &'a SimpleFencer,
) -> impl IntoView {
    if main_fencer == secondary_fencer {
        view! { <td>N</td> }
    } else {
        view! { <td>Y</td> }
    }
}

#[component]
pub fn PoolTableHeader<'a>(fencers: &'a Vec<&'a SimpleFencer>) -> impl IntoView {
    view! {
        <tr>
            <th></th>
            {fencers
                .iter()
                .map(|fencer| view! { <th>{fencer.get_fullname()}</th> })
                .collect::<Vec<_>>()}
        </tr>
    }
}

#[component]
pub fn PoolTableRow<'a>(
    main_fencer: &'a SimpleFencer,
    fencers: &'a [&'a SimpleFencer],
) -> impl IntoView {
    view! {
        <tr>
            <td>{main_fencer.get_fullname()}</td>
            {fencers
                .iter()
                .map(|fencer| {
                    view! { <BoutScoreTableCell main_fencer=main_fencer secondary_fencer=fencer/> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}
