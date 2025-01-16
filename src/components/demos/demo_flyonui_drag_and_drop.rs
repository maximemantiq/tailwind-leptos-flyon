use leptos::prelude::*;

#[component]
pub fn DemoFlyonUIDragAndDrop() -> impl IntoView {
    view! {
        <ul
            id="list-example"
            class="border-base-content/25 divide-base-content/25 flex flex-col divide-y rounded-md border *:cursor-move *:p-3 first:*:rounded-t-md last:*:rounded-b-md"
        >
            <li class="flex items-center gap-3">
                <span class="icon-[tabler--bell] size-4 shrink-0"></span>
                Weekly Insights
                <span class="icon-[tabler--grip-vertical] text-base-content ms-auto size-4 shrink-0"></span>
            </li>
            <li class="flex items-center gap-3">
                <span class="icon-[tabler--cloud-download] size-4 shrink-0"></span>
                Resource Center
                <span class="icon-[tabler--grip-vertical] text-base-content ms-auto size-4 shrink-0"></span>
            </li>
            <li class="flex items-center gap-3">
                <span class="icon-[tabler--users] size-4 shrink-0"></span>
                Team Collaboration
                <span class="icon-[tabler--grip-vertical] text-base-content ms-auto size-4 shrink-0"></span>
            </li>
            <li class="flex items-center gap-3">
                <span class="icon-[tabler--bell] size-4 shrink-0"></span>
                Product Updates
                <span class="icon-[tabler--grip-vertical] text-base-content ms-auto size-4 shrink-0"></span>
            </li>
            <li class="flex items-center gap-3">
                <span class="icon-[tabler--users] size-4 shrink-0"></span>
                Community Forum
                <span class="icon-[tabler--grip-vertical] text-base-content ms-auto size-4 shrink-0"></span>
            </li>
        </ul>

        <script>
            window.addEventListener("load", () => {
                (function () {
                    // Basic example
                    const listExample = document.querySelector("#list-example");

                    if (listExample) {
                        Sortable.create(listExample, {
                            animation: 150,
                            dragClass: "!border-0"
                        })
                    }
                })()
            })
        // Basic example
        </script>
    }
}
