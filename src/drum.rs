use leptos::html;
use leptos::prelude::*;

const ITEM_HEIGHT: i32 = 40;

#[component]
pub fn DrumPicker(
    items: Vec<String>,
    initial_value: String,
    on_change: Callback<String>,
) -> impl IntoView {
    let scroll_ref: NodeRef<html::Div> = NodeRef::new();

    let init_items = items.clone();
    scroll_ref.on_load(move |el| {
        let idx = init_items
            .iter()
            .position(|v| v == &initial_value)
            .unwrap_or(0);
        el.set_scroll_top(idx as i32 * ITEM_HEIGHT);
    });

    let scroll_items = items.clone();
    let on_scroll = move |_| {
        if let Some(el) = scroll_ref.get() {
            let top = el.scroll_top();
            let idx = ((top + ITEM_HEIGHT / 2) / ITEM_HEIGHT) as usize;
            let idx = idx.min(scroll_items.len().saturating_sub(1));
            on_change.run(scroll_items[idx].clone());
        }
    };

    view! {
        <div class="th-drum">
            <div class="th-drum-highlight"></div>
            <div class="th-drum-scroll" node_ref=scroll_ref on:scroll=on_scroll>
                {items
                    .iter()
                    .map(|v| view! { <div class="th-drum-item">{v.clone()}</div> })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
