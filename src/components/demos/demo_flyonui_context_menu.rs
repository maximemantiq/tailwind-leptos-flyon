use leptos::prelude::*;

#[component]
pub fn DemoFlyonUIContextMenu() -> impl IntoView {
    view! {
        <div class="dropdown relative inline-flex rtl:[--placement:bottom-end] [--trigger:contextmenu]">
            <div class="dropdown-toggle py-3 px-4 w-96 h-25 flex justify-center items-center text-sm font-medium rounded-lg border-2 border-dashed border-primary bg-base-100 text-primary shadow-sm">
                Right click
            </div>
            <div
                class="dropdown-menu dropdown-open:opacity-100 hidden min-w-60"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="default"
            >

                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--arrow-back-up] size-5 shrink-0"></span>
                    Reply
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--arrow-back-up-double] size-5 shrink-0"></span>
                    Reply all
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--arrow-forward-up] size-5 shrink-0"></span>
                    Forward
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--send] size-5 shrink-0"></span>
                    Resend
                </button>
                <hr class="border-base-content/25 -mx-2 my-2" />

                <div class="dropdown relative [--offset:15] max-sm:[--placement:bottom-start] [--placement:right-start] [--trigger:hover]">
                    <button
                        type="button"
                        class="dropdown-toggle dropdown-item"
                        role="menuitem"
                        tabindex="-1"
                    >
                        <span class="icon-[tabler--text-plus] size-5 shrink-0"></span>
                        More
                        <span class="icon-[tabler--chevron-right] rtl:rotate-180 size-5 shrink-0 ms-auto"></span>
                    </button>
                    <div
                        class="dropdown-menu dropdown-open:opacity-100 hidden min-w-60 before:w-6 before:absolute before:-start-6 before:top-0 before:h-full after:w-6 after:absolute after:-end-6 after:top-0 after:h-full"
                        role="menu"
                        aria-orientation="vertical"
                        aria-labelledby="default"
                    >
                        <button type="button" class="dropdown-item">
                            <span class="icon-[tabler--clipboard-copy] size-5 shrink-0"></span>
                            Copy conversation
                        </button>
                        <button type="button" class="dropdown-item">
                            <span class="icon-[tabler--archive] size-5 shrink-0"></span>
                            Archive Conversation
                        </button>
                        <button type="button" class="dropdown-item">
                            <span class="icon-[tabler--mail-opened] size-5 shrink-0"></span>
                            Move to Folder
                        </button>
                        <button type="button" class="dropdown-item">
                            <span class="icon-[tabler--star] size-5 shrink-0"></span>
                            Mark as Important
                        </button>
                        <button type="button" class="dropdown-item">
                            <span class="icon-[tabler--bell-off] size-5 shrink-0"></span>
                            Mute Notifications
                        </button>
                    </div>
                </div>
                <hr class="border-base-content/25 -mx-2 my-2" />

                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--mail] size-5 shrink-0"></span>
                    Mark as unread
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--mail-opened] size-5 shrink-0"></span>
                    Mark as read
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--archive] size-5 shrink-0"></span>
                    Archive
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--trash] size-5 shrink-0"></span>
                    Delete
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--alert-circle] size-5 shrink-0"></span>
                    Report Spam
                </button>
                <button type="button" class="dropdown-item">
                    <span class="icon-[tabler--file-export] size-5 shrink-0"></span>
                    Export Conversation
                </button>
            </div>
        </div>
    }
}
