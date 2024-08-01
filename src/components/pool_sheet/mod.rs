use fencing_sport_lib::{
    bout::FencerVs,
    fencer::SimpleFencer,
    pools::{bout_creation::SimpleBoutsCreator, PoolSheet},
};

use leptos::*;

mod bout_list;
use bout_list::BoutList;

mod sheet_table;
use log::info;
use sheet_table::PoolSheetTable;

#[component]
pub fn PoolSheet(fencers: Vec<SimpleFencer>) -> impl IntoView {
    let option_poolsheet = PoolSheet::new(fencers, &SimpleBoutsCreator).ok();

    match option_poolsheet {
        Some(poolsheet) => {
            let (poolsheet_sig, set_poolsheet_sig) = create_signal(poolsheet);

            let set_bout_score = move |fencer_a, fencer_b| {
                set_poolsheet_sig.update(|poolsheet| {
                    poolsheet.update_score(fencer_a, fencer_b).ok();
                })
            };
            let get_fencers = move || {
                poolsheet_sig
                    .with(|sheet| sheet.get_fencers().into_iter().cloned().collect::<Vec<_>>())
            };
            let get_bout_score = move |fencer_vs| {
                poolsheet_sig.with(|sheet| {
                    let bout = sheet.get_bout(&fencer_vs).ok()?;
                    bout.get_scores()
                })
            };
            let get_bout_main_score = move |fencer_main: SimpleFencer, fencer_sec: SimpleFencer| {
                poolsheet_sig.with(|sheet| {
                    let vs = FencerVs::new(fencer_main.clone(), fencer_sec.clone()).unwrap();
                    let bout = sheet.get_bout(&vs).unwrap();
                    bout.get_score(fencer_main)
                })
            };
            let get_versus = move || {
                poolsheet_sig.with(|sheet| {
                    sheet
                        .iter_bouts()
                        .map(|(x, y)| (x.clone()))
                        .collect::<Vec<_>>()
                })
            };
            let versus = get_versus();
            view! {
                <PoolSheetTable fencers=get_fencers get_main_score=get_bout_main_score/>
                <BoutList
                    versus=versus
                    set_score_closure=set_bout_score
                    get_score_closure=get_bout_score
                />
            }
        }
        None => Fragment::new(vec![View::Text(view! { "Goodbyte Poolsheet" })]),
    }
}
