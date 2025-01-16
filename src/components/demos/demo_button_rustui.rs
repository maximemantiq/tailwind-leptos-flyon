use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonRustUI() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="flex gap-4">
            <Button on:click=on_click>"Click Me: " {count}</Button>

            <Button class="bg-sky-500">"Button Sky"</Button>
        </div>
    }
}
