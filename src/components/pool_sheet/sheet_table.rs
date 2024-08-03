use fencing_sport_lib::fencer::{Fencer, SimpleFencer};

use leptos::*;
use log::info;

#[component]
pub fn PoolSheetTable<F, FB>(fencers: F, get_main_score: FB) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
    FB: Fn(SimpleFencer, SimpleFencer) -> Option<u8> + Clone + 'static,
{
    info!("Rendering PoolSheetTable");

    view! {
        <table class="poolsheet-table">
            <PoolTableHeader fencers=fencers.clone()/>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! {
                        <PoolTableRow
                            main_fencer=fencer
                            fencers=fencers.clone()
                            get_main_score=get_main_score.clone()
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </table>
    }
}

#[component]
pub fn TableScoreCell<'a, FG>(
    main_fencer: &'a SimpleFencer,
    secondary_fencer: &'a SimpleFencer,
    get_main_score: FG,
) -> impl IntoView
where
    FG: Fn(SimpleFencer, SimpleFencer) -> Option<u8> + Clone + 'static,
{
    info!("Rendering TableScoreCell");

    let main_fencer = main_fencer.clone();
    let secondary_fencer = secondary_fencer.clone();

    if main_fencer == secondary_fencer {
        view! { <td class="poolsheet-cell-blank">X</td> }
    } else {
        let get_my_score = move || {
            let tmp = match get_main_score(main_fencer.clone(), secondary_fencer.clone()) {
                Some(x) => x.to_string(),
                None => String::from("/"),
            };
            info!("Getting score for {main_fencer:?} - {secondary_fencer:?} = {tmp:?}");
            tmp
        };
        view! { <td class="poolsheet-cell">{get_my_score}</td> }
    }
}

#[component]
pub fn PoolTableHeader<F>(fencers: F) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone,
{
    info!("Rendering PoolTableHeader");

    view! {
        <tr>
            <th></th>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! { <td class="pool-sheet-fencer-second">{fencer.get_fullname()}</td> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}

#[component]
pub fn PoolTableRow<'a, F, FG>(
    main_fencer: &'a SimpleFencer,
    fencers: F,
    get_main_score: FG,
) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
    FG: Fn(SimpleFencer, SimpleFencer) -> Option<u8> + Clone + 'static,
{
    info!("Rendering PoolTableRow");
    view! {
        <tr>
            <td>{main_fencer.get_fullname()}</td>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! {
                        <TableScoreCell
                            main_fencer=&main_fencer
                            secondary_fencer=fencer
                            get_main_score=get_main_score.clone()
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}
