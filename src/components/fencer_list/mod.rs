use std::fmt::format;
use std::str::FromStr;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde_json;

#[component]
pub fn FencerList(submit_fencers: WriteSignal<Vec<String>>) -> impl IntoView {
    let initial_fencers = Vec::new();

    let (fencers, set_fencers) = create_signal(initial_fencers);

    let mut fencer_id = 0;
    let add_fencer = move |_| {
        let fencer_sig = create_signal(fencer_id + 1);
        let fencer_ref: NodeRef<html::Input> = create_node_ref();
        set_fencers.update(move |fencers| fencers.push((fencer_id, fencer_sig, fencer_ref)));
        fencer_id += 1;
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let values: Vec<String> = fencers
            .get()
            .into_iter()
            .map(|(_, _, node_refs)| node_refs().expect("the error").value())
            .collect();

        submit_fencers.update(|val| *val = values);
    };

    view! {
        <div>
            <button on:click=add_fencer>"Add Counter"</button>
            <form on:submit=on_submit>
                <ul>
                    <For
                        // `each` takes any function that returns an iterator
                        // this should usually be a signal or derived signal
                        each=fencers
                        key=|counter| counter.0
                        // `children` receives each item from your `each` iterator
                        // and returns a view
                        children=move |(id, (fencer, set_fencer), fencer_ref)| {
                            view! {
                                <li>
                                    <input type="text" value=fencer node_ref=fencer_ref/>
                                    <button on:click=move |_| {
                                        set_fencers
                                            .update(|counters| {
                                                counters
                                                    .retain(|(counter_id, (signal, _), _)| {
                                                        if counter_id == &id {
                                                            signal.dispose();
                                                        }
                                                        counter_id != &id
                                                    })
                                            });
                                    }>

                                        "Remove"
                                    </button>
                                </li>
                            }
                        }
                    />

                </ul>
                <input type="submit" value="Submit"/>
            </form>
        </div>
    }
}
