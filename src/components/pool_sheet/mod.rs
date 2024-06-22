use std::str::FromStr;
use std::{fmt::format, rc::Rc};

use fencing_sport_lib::{
    bout::{FencerScore, FencerVs},
    fencer::{Fencer, SimpleFencer},
    pools::{PoolSheet, SimpleBoutsCreator},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde_json;

#[component]
pub fn PoolSheet(fencers: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        {move || {
            let fencers: Rc<Vec<SimpleFencer>> = Rc::new(
                fencers
                    .get()
                    .into_iter()
                    .map(|fencer_str| { SimpleFencer::new(fencer_str) })
                    .collect(),
            );
            match PoolSheet::new(&fencers, &SimpleBoutsCreator) {
                Ok(mut poolsheet) => {
                    let test = create_signal(poolsheet);
                    view! {
                        <div>
                            <div class="poolsheet">
                                <table>
                                    <tr>
                                        <td></td>

                                        {fencers
                                            .iter()
                                            .map(|fencer| {
                                                view! {
                                                    <td class="pool-sheet-fencer-second">
                                                        {fencer.get_fullname()}
                                                    </td>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </tr>

                                    {fencers
                                        .iter()
                                        .map(|fencer_main| {
                                            view! {
                                                <tr>
                                                    <td class="pool-sheet-fencer">
                                                        {fencer_main.get_fullname()}
                                                    </td>
                                                    {fencers
                                                        .iter()
                                                        .map(|fencer_second| {
                                                            view! {
                                                                <td
                                                                    class=if fencer_second == fencer_main {
                                                                        "pool-sheet-score-box-null"
                                                                    } else {
                                                                        "pool-sheet-score-box"
                                                                    }

                                                                    id=format!(
                                                                        "{}-{}",
                                                                        fencer_main.get_fullname(),
                                                                        fencer_second.get_fullname(),
                                                                    )
                                                                >

                                                                    {{
                                                                        let vs = FencerVs::new(fencer_main, fencer_second).unwrap();
                                                                        let x = poolsheet
                                                                            .get_bout(&vs)
                                                                            .unwrap()
                                                                            .get_score(fencer_main)
                                                                            .unwrap();
                                                                        x
                                                                    }}

                                                                </td>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()}

                                </table>
                            </div>

                            <ol class="bout-list">

                                {poolsheet
                                    .iter()
                                    .map(|(versus, _)| {
                                        let (bout_score_a, set_bout_score_a) = create_signal(None);
                                        let (bout_score_b, set_bout_score_b) = create_signal(None);
                                        view! {
                                            <li>
                                                {format!(
                                                    "{} vs {}",
                                                    versus.0.get_fullname(),
                                                    versus.1.get_fullname(),
                                                )}
                                                <input
                                                    type="number"
                                                    on:input=move |ev| {
                                                        let test = event_target_value(&ev).parse::<u8>().ok();
                                                        set_bout_score_a(test);
                                                    }
                                                />
                                                <input
                                                    type="number"
                                                    on:input=move |ev| {
                                                        let test = event_target_value(&ev).parse::<u8>().ok();
                                                        set_bout_score_b(test);
                                                    }
                                                />
                                                <p>--- {bout_score_a} - {bout_score_b}</p>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()}

                            </ol>
                        </div>
                    }
                }
                Err(err) => {
                    view! {
                        // let test = create_signal(poolsheet);

                        // let test = create_signal((poolsheet, fencers));

                        <div>
                            <p>{format!("{err:?}")}</p>
                        </div>
                    }
                }
            }
        }}
    }
}
