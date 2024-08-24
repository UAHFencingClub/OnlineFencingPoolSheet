// The PoolTableRow as this warning, but removing the lifetime makes the module error.
// Unsure why TableScoreCell does not have this issue.
#![allow(clippy::needless_lifetimes)]
use fencing_sport_lib::{
    bout::FencerVs,
    fencer::{Fencer, SimpleFencer},
    pools::PoolSheet,
};

use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};
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
    column_count: usize,
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

    let cell_width = 100.0 / (column_count as f32);
    let width_height_style = format!("width: {cell_width:.1}%; height: {cell_width:.1}%;");

    let td_noderef = create_node_ref();
    let span_noderef = create_node_ref();

    let UseElementSizeReturn {
        width: _width,
        height,
    } = use_element_size(td_noderef);

    let get_span_style = move || {
        let height_rounded = height() as usize;
        info!("Test {height_rounded}");
        format!("width: 100%; font-size: {height_rounded}px")
    };

    if main_fencer == secondary_fencer {
        view! { <td class="poolsheet-cell-blank ratio ratio-1x1" style=width_height_style></td> }
    } else {
        let get_my_score = move || {
            let tmp = match get_main_score(main_fencer.clone(), secondary_fencer.clone()) {
                Some(x) => x.to_string(),
                None => String::from(""),
            };
            info!("Getting score for {main_fencer:?} - {secondary_fencer:?} = {tmp:?}");
            tmp
        };
        view! {
            <td
                class="poolsheet-cell  ratio ratio-1x1"
                style=width_height_style
                node_ref=td_noderef
            >
                <span class="poolsheet-cell-text" style=get_span_style node_ref=span_noderef>
                    {get_my_score}
                </span>
            </td>
        }
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
    let len = fencers().len();
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
                            column_count=len
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}
