use std::fmt::format;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use fencing_sport_lib::pools::PoolSheet;

// Modules
mod components;
mod pages;

// use console_log::info;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let (competiors, set_competitors) = create_signal(Vec::<String>::new());
    view! {
        <FencerList submit_fencers=set_competitors/>
        <PoolSheet fencers=competiors />
        // <h1>{move || {format!("{:?}",competiors.get())}}</h1>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn FencerList(submit_fencers: WriteSignal<Vec<String>>) -> impl IntoView {
    let initial_fencers = Vec::new();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (fencers, set_fencers) = create_signal(initial_fencers);

    // let (deug_vec, set_debug_vec) = create_signal("debug_sig".to_string());

    let mut fencer_id = 0;
    let add_fencer = move |_| {
        let fencer_sig = create_signal(fencer_id + 1);
        let fencer_ref: NodeRef<html::Input> = create_node_ref();
        set_fencers.update(move |fencers| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            fencers.push((fencer_id, fencer_sig, fencer_ref))
        });
        // increment the ID so it's always unique
        fencer_id += 1;
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let values: Vec<String> = fencers.get().into_iter().map(|(_, _, node_refs)| node_refs().expect("the error").value()).collect();
    
        // // here, we'll extract the value from the input
        // let value = input_element()
        //     // event handlers can only fire after the view
        //     // is mounted to the DOM, so the `NodeRef` will be `Some`
        //     .expect("<input> should be mounted")
        //     // `leptos::HtmlElement<html::Input>` implements `Deref`
        //     // to a `web_sys::HtmlInputElement`.
        //     // this means we can call`HtmlInputElement::value()`
        //     // to get the current value of the input
        //     .value();
        // set_name(value);
        // set_debug_vec(format!("{values:?}"))
        submit_fencers.update(|val| *val=values);

    };

    view! {
        <div>
            <button on:click=add_fencer>
                "Add Counter"
            </button>
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
                                    // <button
                                    //     on:click=move |_| set_fencer.update(|n| *n += 1)
                                    // >
                                    //     {fencer}
                                    // </button>
                                    <input type="text"
                                        value=fencer
                                        node_ref=fencer_ref
                                    />
                                    <button
                                        on:click=move |_| {
                                            set_fencers.update(|counters| {
                                                counters.retain(|(counter_id, (signal, _), _)| {
                                                    // NOTE: in this example, we are creating the signals
                                                    // in the scope of the parent. This means the memory used to
                                                    // store them will not be reclaimed until the parent component
                                                    // is unmounted. Here, we're removing the signal early (i.e, before
                                                    // the DynamicList is unmounted), so we manually dispose of the signal
                                                    // to avoid leaking memory.
                                                    //
                                                    // This is only necessary in an example with nested signals like this one.
                                                    if counter_id == &id {
                                                        signal.dispose();
                                                    }
                                                    counter_id != &id
                                                })
                                            });
                                        }
                                    >
                                        "Remove"
                                    </button>
                                </li>
                            }
                        }
                    />
                </ul>
                <input type="submit" value="Submit"/>
            </form>
            // <p> {deug_vec} </p>
        </div>
    }
}

#[component]
fn PoolSheet(fencers: ReadSignal<Vec<String>>) -> impl IntoView {

    view! {
        <ul>
        {move || {
            fencers.get().into_iter()
            .map(|n| view! { <li>{n}</li>})
            .collect::<Vec<_>>()
        }}
        </ul>
    }
}