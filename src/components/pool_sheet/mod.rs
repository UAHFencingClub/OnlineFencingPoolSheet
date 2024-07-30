use std::str::FromStr;
use std::{fmt::format, rc::Rc};

use fencing_sport_lib::bout::Bout;
use fencing_sport_lib::cards::Cards;
use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    fencer::{Fencer, SimpleFencer},
    pools::{bout_creation::SimpleBoutsCreator, PoolBoutIter, PoolSheet, PoolSheetBout},
};
use html::B;
use itertools::Itertools;
use leptos::*;
use leptos_dom::Text;
use leptos_meta::*;
use leptos_router::*;
use log::{info, log};
use serde_json;

const POOL_MAX_SCORE: u8 = 5;

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
                let set_bout_score = move |fencer_a, fencer_b| {
                    set_poolsheet_sig
                        .update(|poolsheet| {
                            match poolsheet {
                                Some(poolsheet) => {
                                    info!("Setting the score");
                                    poolsheet.update_score(fencer_a, fencer_b).ok();
                                }
                                None => {}
                            }
                        })
                };
                let get_bout_score = move |fencer_vs| {
                    poolsheet_sig
                        .with(|poolsheet_option| {
                            match poolsheet_option {
                                Some(poolsheet) => {
                                    let bout = poolsheet.get_bout(&fencer_vs).ok()?;
                                    bout.get_scores()
                                }
                                None => None,
                            }
                        })
                };
                poolsheet_sig
                    .with(|poolsheet_option| {
                        match poolsheet_option {
                            Some(poolsheet) => {
                                view! {
                                    <PoolSheetTable poolsheet=poolsheet/>
                                    <BoutList
                                        bouts=poolsheet.iter_bouts()
                                        set_score_closure=set_bout_score
                                        get_score_closure=get_bout_score
                                    />
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
fn BoutList<'a, FS, FG>(
    bouts: PoolBoutIter<'a, SimpleFencer>,
    set_score_closure: FS,
    get_score_closure: FG,
) -> impl IntoView
where
    FS: Fn(FencerScore<SimpleFencer, SimpleFencer>, FencerScore<SimpleFencer, SimpleFencer>)
        + Clone
        + 'static,
    FG: Fn(FencerVs<SimpleFencer, SimpleFencer>) -> Option<(u8, u8)> + Clone + 'static,
{
    view! {
        <ol>
            {bouts
                .into_iter()
                .map(|(vs, bout)| {
                    view! {
                        <li>
                            <div>{vs.0.get_fullname()} vs {vs.1.get_fullname()}</div>
                            <BoutListInputItem
                                fencer_a=bout.get_fencers().0.clone()
                                fencer_b=bout.get_fencers().1.clone()
                                set_sheet_score=set_score_closure.clone()
                            />
                            <p>

                                {
                                    let get = get_score_closure.clone();
                                    let fa = vs.0.as_ref().clone();
                                    let fb = vs.1.as_ref().clone();
                                    let x = FencerVs::new(fa, fb).unwrap();
                                    let scores = get(x);
                                    format!("{scores:?}")
                                }

                            </p>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ol>
    }
}

#[component]
fn BoutListInputItem<F>(
    fencer_a: SimpleFencer,
    fencer_b: SimpleFencer,
    set_sheet_score: F,
) -> impl IntoView
where
    F: Fn(FencerScore<SimpleFencer, SimpleFencer>, FencerScore<SimpleFencer, SimpleFencer>)
        + Clone
        + 'static,
{
    let (score, set_score) = create_signal((None::<u8>, None::<u8>));

    let fencer_aa = fencer_a.clone();
    let fencer_ab = fencer_b.clone();
    let fencer_ba = fencer_a;
    let fencer_bb = fencer_b;

    let set_sheet_score_a = set_sheet_score.clone();
    let set_sheet_score_b = set_sheet_score.clone();
    view! {
        <input
            type="number"
            on:input=move |ev| {
                let score_a = match event_target_value(&ev).parse::<u8>().ok() {
                    Some(score) => {
                        if score > POOL_MAX_SCORE { Some(POOL_MAX_SCORE) } else { Some(score) }
                    }
                    None => None,
                };
                let score_b = score.get().1;
                set_score((score_a, score_b));
                if let Some((a, b)) = score_a.zip(score_b) {
                    let fencer_a = FencerScore::new(fencer_aa.clone(), a, Cards::default());
                    let fencer_b = FencerScore::new(fencer_ab.clone(), b, Cards::default());
                    set_sheet_score_a(fencer_a, fencer_b);
                }
            }

            prop:value=move || { score.get().0 }
        />
        "vs."
        <input
            type="number"
            on:input=move |ev| {
                let score_b = match event_target_value(&ev).parse::<u8>().ok() {
                    Some(score) => {
                        if score > POOL_MAX_SCORE { Some(POOL_MAX_SCORE) } else { Some(score) }
                    }
                    None => None,
                };
                let score_a = score.get().0;
                set_score((score_a, score_b));
                if let Some((a, b)) = score_a.zip(score_b) {
                    let fencer_a = FencerScore::new(fencer_ba.clone(), a, Cards::default());
                    let fencer_b = FencerScore::new(fencer_bb.clone(), b, Cards::default());
                    set_sheet_score_b(fencer_a, fencer_b);
                }
            }

            prop:value=move || { score.get().1 }
        />
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
