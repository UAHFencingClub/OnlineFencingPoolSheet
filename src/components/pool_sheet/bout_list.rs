use std::rc::Rc;

use ev::Event;
use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    cards::Cards,
    fencer::{Fencer, SimpleFencer},
    pools::{PoolBoutIter, PoolSheetFencerScore, PoolSheetVersus},
};

use leptos::*;

const POOL_MAX_SCORE: u8 = 5;

#[component]
pub fn BoutList<FS, FG>(
    versus: Vec<PoolSheetVersus<SimpleFencer>>,
    set_score_closure: FS,
    get_score_closure: FG,
) -> impl IntoView
where
    FS: Fn(PoolSheetFencerScore<SimpleFencer>, PoolSheetFencerScore<SimpleFencer>)
        + Clone
        + 'static,
    FG: Fn(PoolSheetVersus<SimpleFencer>) -> Option<(u8, u8)> + Clone + 'static,
{
    view! {
        <ol>
            {versus
                .into_iter()
                .map(|vs| {
                    let local_vs = vs.clone();
                    let get_score_closure_local = get_score_closure.clone();
                    let get_my_score = move || { get_score_closure_local(local_vs.clone()) };
                    view! {
                        <li>
                            <div>{vs.0.get_fullname()} vs {vs.1.get_fullname()}</div>
                            <BoutListInputItem
                                versus=vs.clone()
                                set_sheet_score=set_score_closure.clone()
                                get_sheet_score=get_my_score.clone()
                            />
                            <p>

                                {
                                    let scores = get_my_score();
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
pub fn BoutListInputItem<FG, FS>(
    versus: PoolSheetVersus<SimpleFencer>,
    set_sheet_score: FG,
    get_sheet_score: FS,
) -> impl IntoView
where
    FG: Fn(PoolSheetFencerScore<SimpleFencer>, PoolSheetFencerScore<SimpleFencer>)
        + Clone
        + 'static,
    FS: Fn() -> Option<(u8, u8)> + Clone + 'static,
{
    let (score, set_score) = create_signal((None::<u8>, None::<u8>));

    let fencer_aa = versus.0.clone();
    let fencer_ab = versus.1.clone();
    let fencer_ba = versus.0.clone();
    let fencer_bb = versus.1.clone();

    let set_sheet_score_a = set_sheet_score.clone();
    let set_sheet_score_b = set_sheet_score.clone();

    let get_sheet_score_a1 = get_sheet_score.clone();
    let get_sheet_score_b1 = get_sheet_score.clone();

    let get_sheet_score_a2 = get_sheet_score.clone();
    let get_sheet_score_b2 = get_sheet_score.clone();
    view! {
        <input
            type="number"
            on:input=move |ev| {
                let score_a = parse_score_from_event(&ev);
                let score_b = get_sheet_score_a1().map(|x| x.1);
                if let Some((a, b)) = score_a.zip(score_b) {
                    let fencer_a = FencerScore::new(fencer_aa.clone(), a, Cards::default());
                    let fencer_b = FencerScore::new(fencer_ab.clone(), b, Cards::default());
                    set_sheet_score_a(fencer_a, fencer_b);
                }
            }

            prop:value=move || { get_sheet_score_a2().map(|x| x.0) }
        />
        "vs."
        <input
            type="number"
            on:input=move |ev| {
                let score_b = parse_score_from_event(&ev);
                let score_a = get_sheet_score_b1().map(|x| x.0);
                if let Some((a, b)) = score_a.zip(score_b) {
                    let fencer_a = FencerScore::new(fencer_ba.clone(), a, Cards::default());
                    let fencer_b = FencerScore::new(fencer_bb.clone(), b, Cards::default());
                    set_sheet_score_b(fencer_a, fencer_b);
                }
            }

            prop:value=move || { get_sheet_score_b2().map(|x| x.1) }
        />
    }
}

fn parse_score_from_event(ev: &Event) -> Option<u8> {
    match event_target_value(ev).parse::<u8>().ok() {
        Some(score) => {
            if score > POOL_MAX_SCORE {
                Some(POOL_MAX_SCORE)
            } else {
                Some(score)
            }
        }
        None => None,
    }
}
