use std::rc::Rc;

use ev::Event;
use fencing_sport_lib::{
    bout::FencerScore,
    cards::Cards,
    fencer::{Fencer, SimpleFencer},
    pools::{PoolSheet, PoolSheetVersus},
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

    let versus_len = versus.len();

    view! {
        <ol>
            {versus
                .into_iter()
                .enumerate()
                .map(|(index, vs)| {
                    view! {
                        <li class="bout-list-item container">
                            <BoutListItemLabel
                                versus_0=vs.0.get_fullname()
                                versus_1=vs.1.get_fullname()
                            />
                            <BoutListInputItem versus=vs.clone() poolsheet_sigs=poolsheet_sigs/>
                        </li>
                        {if index != versus_len - 1 {
                            view! { <hr/> }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    }
                })
                .collect::<Vec<_>>()}
        </ol>
    }
}

#[component]
pub fn BoutListItemLabel(versus_0: String, versus_1: String) -> impl IntoView {
    view! {
        <div class="bout-list-versus row justify-content-center text-center">
            <div class="col d-flex align-items-center justify-content-end flex-grow-1">
                {versus_0}
            </div>
            <div
                class="col-auto d-flex align-items-center justify-content-center flex-shrink-0"
                style="width: 50px;"
            >
                "vs."
            </div>
            <div class="col d-flex align-items-center justify-content-start flex-grow-1">
                {versus_1}
            </div>
        </div>
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
        <div class="bout-list-inputs row justify-content-center text-center mt-2">
            <div class="col-auto d-flex align-items-center justify-content-center flex-shrink-0">
                <input
                    type="checkbox"
                    class="bout-list-check"
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
                    class="bout-list-input"
                    on:input=move |ev| {
                        let score = parse_score_from_event(&ev);
                        set_scores_a(fencer_a1.clone(), score)
                    }

                    prop:value=move || { get_score_a(fencer_a2.clone()) }
                />
            </div>
            <div class="col-auto d-flex align-items-center justify-content-center flex-shrink-0">
                " - "
            </div>
            <div class="col-auto d-flex align-items-center justify-content-center flex-shrink-0">
                <input
                    type="checkbox"
                    class="bout-list-check"
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
                    class="bout-list-input"
                    on:input=move |ev| {
                        let score = parse_score_from_event(&ev);
                        set_scores_b(fencer_b1.clone(), score)
                    }

                    prop:value=move || { get_score_b(fencer_b2.clone()) }
                />
            </div>
        </div>
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
