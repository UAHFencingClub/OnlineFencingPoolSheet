use fencing_sport_lib::{
    bout::FencerVs,
    fencer::SimpleFencer,
    pools::{bout_creation::SimpleBoutsCreator, PoolResults, PoolSheet, PoolSheetError},
};

use leptos::*;

mod bout_list;
use bout_list::BoutList;

mod sheet_table;
use leptos_dom::Text;
use log::info;
use sheet_table::PoolSheetTable;

#[component]
pub fn PoolSheet<F>(fencers: Vec<SimpleFencer>, on_complete: F) -> impl IntoView
where
    F: Fn(Result<PoolResults<SimpleFencer>, PoolSheetError>) + 'static,
{
    let option_poolsheet = PoolSheet::new(fencers, &SimpleBoutsCreator);

    match option_poolsheet {
        Ok(poolsheet) => {
            let poolsheet_signals = create_signal(poolsheet);

            // let set_bout_score = move |fencer_a, fencer_b| {
            //     set_poolsheet_sig.update(|poolsheet| {
            //         poolsheet.update_score(fencer_a, fencer_b).ok();
            //     })
            // };
            let get_fencers = move || {
                poolsheet_signals
                    .0
                    .with(|sheet| sheet.get_fencers().into_iter().cloned().collect::<Vec<_>>())
            };
            // let get_bout_score = move |fencer_vs| {
            //     poolsheet_sig.with(|sheet| {
            //         let bout = sheet.get_bout(&fencer_vs).ok()?;
            //         bout.get_scores()
            //     })
            // };
            // let get_bout_main_score = move |fencer_main: SimpleFencer, fencer_sec: SimpleFencer| {
            //     info!("Getting main score for {fencer_main:?} {fencer_sec:?}");
            //     poolsheet_sig.with(|sheet| {
            //         let vs = FencerVs::new(fencer_main.clone(), fencer_sec.clone()).unwrap();
            //         let bout = sheet.get_bout(&vs).unwrap();
            //         bout.get_score(fencer_main)
            //     })
            // };
            let get_versus = move || {
                poolsheet_signals.0.with(|sheet| {
                    sheet
                        .iter_bouts()
                        .map(|(vs, _)| (vs.clone()))
                        .collect::<Vec<_>>()
                })
            };
            // let versus = get_versus();
            view! {
                <div class="poolsheet-table-div">
                    <PoolSheetTable fencers=get_fencers poolsheet_sigs=poolsheet_signals/>
                </div>
                <div class="poolsheet-bouts-div">
                    <BoutList versus=get_versus() poolsheet_sigs=poolsheet_signals/>

                    <button
                        id="get-poolsheets-results"
                        on:click=move |_| {
                            poolsheet_signals.0.with(|sheet| { on_complete(sheet.finish()) });
                        }
                    >

                        Get Results
                    </button>
                </div>
            }
            .into_view()
        }

        Err(e) => view! { <p>{format!("Error {:?}", e)}</p> }.into_view(),
    }
}
// Err(e) => Fragment::new(vec![View::Text(Text::new(format!("Error {:?}", e).into()))]),
