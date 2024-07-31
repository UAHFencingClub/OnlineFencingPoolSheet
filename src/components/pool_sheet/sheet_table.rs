use fencing_sport_lib::fencer::{Fencer, SimpleFencer};

use leptos::*;

#[component]
pub fn PoolSheetTable<F>(fencers: F) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
{
    view! {
        <table>
            <PoolTableHeader fencers=fencers.clone()/>
            {fencers()
                .iter()
                .map(|fencer| view! { <PoolTableRow main_fencer=fencer fencers=fencers.clone()/> })
                .collect::<Vec<_>>()}
        </table>
    }
}

#[component]
pub fn BoutScoreTableCell<'a>(
    main_fencer: &'a SimpleFencer,
    secondary_fencer: &'a SimpleFencer,
) -> impl IntoView {
    if main_fencer == secondary_fencer {
        view! { <td>N</td> }
    } else {
        view! { <td>Y</td> }
    }
}

#[component]
pub fn PoolTableHeader<F>(fencers: F) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone,
{
    view! {
        <tr>
            <th></th>
            {fencers()
                .iter()
                .map(|fencer| view! { <th>{fencer.get_fullname()}</th> })
                .collect::<Vec<_>>()}
        </tr>
    }
}

#[component]
pub fn PoolTableRow<'a, F>(main_fencer: &'a SimpleFencer, fencers: F) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
{
    view! {
        <tr>
            <td>{main_fencer.get_fullname()}</td>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! { <BoutScoreTableCell main_fencer=&main_fencer secondary_fencer=fencer/> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}
