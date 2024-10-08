use std::fmt::Display;

use fencing_sport_lib::fencer::SimpleFencer;
use indexmap::IndexSet;
use leptos::*;

#[derive(Debug, Clone, Copy)]
pub enum FencerListError {
    DuplicateFencer,
    NoFencers,
}

impl Display for FencerListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateFencer => write!(f, "fencers with the same name were input"),
            Self::NoFencers => write!(f, "no fencers were input"),
        }
    }
}

impl std::error::Error for FencerListError {}

#[component]
pub fn FencerList(
    submit_fencers: WriteSignal<Result<IndexSet<SimpleFencer>, FencerListError>>,
) -> impl IntoView {
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

        let values: Vec<SimpleFencer> = fencers
            .get()
            .into_iter()
            .map(|(_, _, node_refs)| node_refs().expect("the error").value())
            .map(SimpleFencer::new)
            .collect();

        let mut set = IndexSet::new();
        for fencer in values {
            if !set.insert(fencer) {
                submit_fencers.update(|val| *val = Err(FencerListError::DuplicateFencer));
                return;
            }
        }

        if set.is_empty() {
            submit_fencers.update(
                |val: &mut Result<IndexSet<SimpleFencer>, FencerListError>| {
                    *val = Err(FencerListError::NoFencers)
                },
            );
            return;
        }

        submit_fencers.update(|val| *val = Ok(set));
    };

    view! {
        <div>
            <button on:click=add_fencer>"Add Fencer"</button>
            <form on:submit=on_submit>
                <ul>
                    <For
                        // `each` takes any function that returns an iterator
                        // this should usually be a signal or derived signal
                        each=fencers
                        key=|counter| counter.0
                        // `children` receives each item from your `each` iterator
                        // and returns a view
                        children=move |(id, (fencer, _set_fencer), fencer_ref)| {
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
                <button type="submit">Submit</button>
            </form>
        </div>
    }
}
