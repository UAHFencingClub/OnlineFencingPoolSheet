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

            let (complete_sig, set_complete) = create_signal(Vec::new());

            view! {
                <div id="poolsheet-table-div" class="col-sm-6">
                    <PoolSheetTable fencers=get_fencers poolsheet_sigs=poolsheet_signals/>
                </div>
                <div id="poolsheet-bouts-div" class="col-sm-6">
                    <BoutList
                        versus=get_versus()
                        poolsheet_sigs=poolsheet_signals
                        complete_sig=complete_sig
                    />

                    <button
                        id="get-poolsheets-results"
                        on:click=move |_| {
                            poolsheet_signals
                                .0
                                .with(|sheet| {
                                    let finished_sheet = sheet.finish();
                                    match finished_sheet {
                                        Err(PoolSheetError::PoolNotComplete(ref err_bouts)) => {
                                            set_complete.set(err_bouts.clone())
                                        }
                                        _ => {}
                                    }
                                    on_complete(finished_sheet);
                                });
                        }
                    >

                        Get Results
                    </button>
                </div>
            }
            .into_view()
        }

        Err(err) => view! { <p>{format!("Error: {err}")}</p> }.into_view(),
    }
}
