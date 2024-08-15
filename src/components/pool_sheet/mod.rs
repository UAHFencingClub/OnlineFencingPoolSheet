use fencing_sport_lib::{
    fencer::SimpleFencer,
    pools::{bout_creation::SimpleBoutsCreator, PoolResults, PoolSheet, PoolSheetError},
};
use leptos::*;
mod bout_list;
use bout_list::BoutList;
mod sheet_table;
use indexmap::IndexSet;
use sheet_table::PoolSheetTable;

#[component]
pub fn PoolSheet<F>(fencers: IndexSet<SimpleFencer>, on_complete: F) -> impl IntoView
where
    F: Fn(Result<PoolResults<SimpleFencer>, PoolSheetError>) + 'static,
{
    let option_poolsheet = PoolSheet::new(fencers, &SimpleBoutsCreator);

    match option_poolsheet {
        Ok(poolsheet) => {
            let poolsheet_signals = create_signal(poolsheet);

            let get_fencers = move || {
                poolsheet_signals
                    .0
                    .with(|sheet| sheet.get_fencers().into_iter().cloned().collect::<Vec<_>>())
            };

            let get_versus = move || {
                poolsheet_signals.0.with(|sheet| {
                    sheet
                        .iter_bouts()
                        .map(|(vs, _)| (vs.clone()))
                        .collect::<Vec<_>>()
                })
            };

            view! {
                <div id="poolsheet-table-div">
                    <PoolSheetTable fencers=get_fencers poolsheet_sigs=poolsheet_signals/>
                </div>
                <div id="poolsheet-bouts-div">
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
