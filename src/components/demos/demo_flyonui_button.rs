use leptos::prelude::*;

#[component]
pub fn DemoFlyonUIButton() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <button class="btn">Default</button>
            <button class="btn btn-primary">Primary</button>
            <button class="btn btn-secondary">Secondary</button>
            <button class="btn btn-accent">Accent</button>
            <button class="btn btn-info">Info</button>
            <button class="btn btn-success">Success</button>
            <button class="btn btn-warning">Warning</button>
            <button class="btn btn-error">Error</button>
        </div>
    }
}
