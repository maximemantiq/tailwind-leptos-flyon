use leptos::prelude::*;

use crate::components::demos::{
    demo_button_rustui::DemoButtonRustUI, demo_flyonui_button::DemoFlyonUIButton,
    demo_flyonui_context_menu::DemoFlyonUIContextMenu,
    demo_flyonui_drag_and_drop::DemoFlyonUIDragAndDrop,
};

#[component]
pub fn PageHome() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-4">
            <h1>"Welcome to Leptos!"</h1>

            <DemoButtonRustUI />
            // <DemoButtonBootstrap />
            <DemoFlyonUIButton />

            // <DemoFlyonUISidebar />

            <DemoFlyonUIContextMenu />

            <DemoFlyonUIDragAndDrop />
        </div>
    }
}
