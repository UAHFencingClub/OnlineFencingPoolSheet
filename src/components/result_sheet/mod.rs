use fencing_sport_lib::{fencer::SimpleFencer, pools::PoolResults};
use leptos::*;

#[component]
pub fn PoolResultTable(pool_results: PoolResults<SimpleFencer>) -> impl IntoView {
    view! {
        <ol>
            {pool_results
                .iter()
                .enumerate()
                .map(|(index, result)| view! { <li>{index} - {format!("{:?}", result.1)}</li> })
                .collect_view()}
        </ol>
    }
}
