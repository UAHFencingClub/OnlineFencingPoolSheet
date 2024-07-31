use std::rc::Rc;

use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    cards::Cards,
    fencer::{Fencer, SimpleFencer},
    pools::PoolBoutIter,
};

use leptos::*;

const POOL_MAX_SCORE: u8 = 5;

#[component]
pub fn BoutList<FV, FS, FG>(
    versus: FV,
    set_score_closure: FS,
    get_score_closure: FG,
) -> impl IntoView
where
    FS: Fn(FencerScore<SimpleFencer, SimpleFencer>, FencerScore<SimpleFencer, SimpleFencer>)
        + Clone
        + 'static,
    FG: Fn(FencerVs<SimpleFencer, SimpleFencer>) -> Option<(u8, u8)> + Clone + 'static,
    FV: Fn() -> Vec<FencerVs<SimpleFencer, Rc<SimpleFencer>>> + Clone + 'static,
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
pub fn BoutListInputItem<F>(
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
