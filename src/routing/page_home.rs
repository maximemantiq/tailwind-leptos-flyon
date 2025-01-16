use leptos::prelude::*;

use crate::components::demos::{
    demo_button_rustui::DemoButtonRustUI, demo_flyonui_button::DemoFlyonUIButton,
};

#[component]
pub fn PageHome() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-4">
            <h1>"Welcome to Leptos!"</h1>

            <DemoButtonRustUI />
            // <DemoButtonBootstrap />
            <DemoFlyonUIButton />

        </div>
    }
}
