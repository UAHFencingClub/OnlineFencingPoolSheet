use std::rc::Rc;

use ev::Event;
use fencing_sport_lib::{
    bout::FencerScore,
    cards::Cards,
    fencer::{Fencer, SimpleFencer},
    pools::{PoolSheet, PoolSheetFencerScore, PoolSheetVersus},
};

use leptos::*;
use log::info;

const POOL_MAX_SCORE: u8 = 5;

#[component]
pub fn BoutList(
    versus: Vec<PoolSheetVersus<SimpleFencer>>,
    poolsheet_sigs: (
        ReadSignal<PoolSheet<SimpleFencer>>,
        WriteSignal<PoolSheet<SimpleFencer>>,
    ),
) -> impl IntoView {
    info!("Rendering BoutList");

    view! {
        <ol>
            {versus
                .into_iter()
                .map(|vs| {
                    view! {
                        // let local_vs = vs.clone();
                        // let get_score_closure_local = get_score_closure.clone();
                        // let get_my_score = move || { get_score_closure_local(local_vs.clone()) };
                        <li>
                            <div>{vs.0.get_fullname()} vs. {vs.1.get_fullname()}</div>
                            <BoutListInputItem versus=vs.clone() poolsheet_sigs=poolsheet_sigs/>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ol>
    }
}

#[component]
pub fn BoutListInputItem(
    versus: PoolSheetVersus<SimpleFencer>,
    poolsheet_sigs: (
        ReadSignal<PoolSheet<SimpleFencer>>,
        WriteSignal<PoolSheet<SimpleFencer>>,
    ),
) -> impl IntoView {
    info!("Rendering BoutListInputItem");
    // let (score, set_score) = create_signal((None::<u8>, None::<u8>));

    let (read_poolsheet, write_poolsheet) = poolsheet_sigs;

    let vs_get = versus.clone();
    let vs_set = versus.clone();
    let vs_setp = versus.clone();

    let get_score = move |fencer: Rc<SimpleFencer>| {
        read_poolsheet.with(|sheet| sheet.get_bout(&vs_get.clone()).unwrap().get_score(fencer))
    };

    let get_score_a = get_score.clone();
    let get_score_b = get_score.clone();

    let set_score = move |fencer: Rc<SimpleFencer>, score: Option<u8>| {
        write_poolsheet.update(|sheet| {
            let bout = sheet.get_bout_mut(&vs_set.clone()).unwrap();
            match score {
                Some(score) => {
                    let score = FencerScore::new(fencer, score, Cards::default());
                    bout.set_score(score).unwrap();
                }
                None => bout.unset_score(fencer).unwrap(),
            }
        })
    };

    let set_priority = move |fencer: Option<Rc<SimpleFencer>>| {
        write_poolsheet.update(|sheet| {
            let bout = sheet.get_bout_mut(&vs_setp.clone()).unwrap();
            bout.set_priority(fencer).unwrap();
        })
    };

    let set_scores_a = set_score.clone();
    let set_scores_b = set_score.clone();

    let set_priority_a = set_priority.clone();
    let set_priority_b = set_priority.clone();

    let fencer_a1 = versus.0.clone();
    let fencer_a2 = versus.0.clone();
    let fencer_a3 = versus.0.clone();
    let fencer_b1 = versus.1.clone();
    let fencer_b2 = versus.1.clone();
    let fencer_b3 = versus.1.clone();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum LR {
        Left,
        Right,
        None,
    }

    let (get_check, set_check) = create_signal(LR::None);

    view! {
        <input
            type="checkbox"
            on:input=move |ev| {
                if event_target_checked(&ev) {
                    set_check.set(LR::Left);
                    set_priority_a(Some(fencer_a3.clone()));
                } else {
                    set_check.set(LR::None);
                    set_priority_a(None);
                }
            }

            prop:checked=move || { get_check.get() == LR::Left }
        />
        <input
            type="number"
            on:input=move |ev| {
                let score = parse_score_from_event(&ev);
                set_scores_a(fencer_a1.clone(), score)
            }

            prop:value=move || { get_score_a(fencer_a2.clone()) }
        />
        " vs. "
        <input
            type="checkbox"
            on:input=move |ev| {
                if event_target_checked(&ev) {
                    set_check.set(LR::Right);
                    set_priority_b(Some(fencer_b3.clone()));
                } else {
                    set_check.set(LR::None);
                    set_priority_b(None);
                }
            }

            prop:checked=move || { get_check.get() == LR::Right }
        />
        <input
            type="number"
            on:input=move |ev| {
                let score = parse_score_from_event(&ev);
                set_scores_b(fencer_b1.clone(), score)
            }

            prop:value=move || { get_score_b(fencer_b2.clone()) }
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
