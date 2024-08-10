use fencing_sport_lib::{
    bout::FencerVs,
    fencer::{Fencer, SimpleFencer},
    pools::PoolSheet,
};

use leptos::*;
use log::info;

#[component]
pub fn PoolSheetTable<F>(
    fencers: F,
    poolsheet_sigs: (
        ReadSignal<PoolSheet<SimpleFencer>>,
        WriteSignal<PoolSheet<SimpleFencer>>,
    ),
) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
{
    info!("Rendering PoolSheetTable");

    // let text_height = 100 / (fencers().len() + 2);

    // let table_style_tag = format!("font-size:{text_height}vw");
    let table_style_tag = "";

    view! {
        <table class="poolsheet-table" style=table_style_tag>
            <PoolTableHeader fencers=fencers.clone()/>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! {
                        <PoolTableRow
                            main_fencer=fencer
                            fencers=fencers.clone()
                            poolsheet_sigs=poolsheet_sigs
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </table>
    }
}

#[component]
pub fn TableScoreCell<'a>(
    main_fencer: &'a SimpleFencer,
    secondary_fencer: &'a SimpleFencer,
    poolsheet_sigs: (
        ReadSignal<PoolSheet<SimpleFencer>>,
        WriteSignal<PoolSheet<SimpleFencer>>,
    ),
) -> impl IntoView {
    info!("Rendering TableScoreCell");

    let main_fencer = main_fencer.clone();
    let secondary_fencer = secondary_fencer.clone();

    let get_main_score = move |fencer_main: SimpleFencer, fencer_sec: SimpleFencer| {
        poolsheet_sigs.0.with(|sheet| {
            let vs = FencerVs::new(fencer_main.clone(), fencer_sec.clone()).unwrap();
            let bout = sheet.get_bout(&vs).unwrap();
            bout.get_score(fencer_main)
        })
    };

    if main_fencer == secondary_fencer {
        view! { <td class="poolsheet-cell-blank"></td> }
    } else {
        let get_my_score = move || {
            let tmp = match get_main_score(main_fencer.clone(), secondary_fencer.clone()) {
                Some(x) => x.to_string(),
                None => String::from(""),
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
                    view! { <td class="poolsheet-fencer-second">{fencer.get_fullname()}</td> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}

#[component]
pub fn PoolTableRow<'a, F>(
    main_fencer: &'a SimpleFencer,
    fencers: F,
    poolsheet_sigs: (
        ReadSignal<PoolSheet<SimpleFencer>>,
        WriteSignal<PoolSheet<SimpleFencer>>,
    ),
) -> impl IntoView
where
    F: Fn() -> Vec<SimpleFencer> + Clone + 'static,
{
    info!("Rendering PoolTableRow");
    view! {
        <tr>
            <td class="poolsheet-fencer-main">{main_fencer.get_fullname()}</td>
            {fencers()
                .iter()
                .map(|fencer| {
                    view! {
                        <TableScoreCell
                            main_fencer=&main_fencer
                            secondary_fencer=fencer
                            poolsheet_sigs=poolsheet_sigs
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}
