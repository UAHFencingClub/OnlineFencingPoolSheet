use fencing_sport_lib::{
    fencer::{Fencer, SimpleFencer},
    pools::PoolResults,
};
use leptos::*;

#[component]
pub fn PoolResultTable(pool_results: PoolResults<SimpleFencer>) -> impl IntoView {
    view! {
        <table class="poolresults-table">
            <tr>
                <th>"Fencer"</th>
                <th>TS</th>
                <th>TR</th>
                <th>Ind</th>
                <th>V</th>
                <th>Pl</th>
            </tr>
            {pool_results
                .iter()
                .map(|result| {
                    view! {
                        <tr>
                            <td>{result.1.fencer().get_fullname()}</td>
                            <td>{*result.1.touches_scored()}</td>
                            <td>{*result.1.touches_recieved()}</td>
                            <td>{*result.1.indicator()}</td>
                            <td>{*result.1.victories()}</td>
                            <td>{result.1.place().to_string()}</td>
                        </tr>
                    }
                })
                .collect_view()}
        </table>
    }
}
