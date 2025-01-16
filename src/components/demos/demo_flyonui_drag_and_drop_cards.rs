use leptos::prelude::*;

#[component]
pub fn DemoFlyonUIDragAndDropCards() -> impl IntoView {
    view! {
        <script src="/dragAndDropCards.js"></script>

        <div id="card-list" class="grid grid-cols-2 gap-6 *:cursor-move">
            <div class="stats max-sm:w-full">
                <div class="stat">
                    <div class="avatar placeholder">
                        <div class="bg-success/20 text-success size-10 rounded-full">
                            <span class="icon-[tabler--package] size-6"></span>
                        </div>
                    </div>
                    <div class="stat-value mb-1">Order</div>
                    <div class="stat-title">7,500 of 10,000 orders</div>
                    <div
                        class="progress bg-success/10 h-2"
                        role="progressbar"
                        aria-label="Order Progressbar"
                        aria-valuenow="75"
                        aria-valuemin="0"
                        aria-valuemax="100"
                    >
                        <div class="progress-bar progress-success w-3/4"></div>
                    </div>
                </div>
            </div>

            <div class="stats max-sm:w-full">
                <div class="stat">
                    <div class="avatar placeholder">
                        <div class="bg-warning/20 text-warning size-10 rounded-full">
                            <span class="icon-[tabler--cash] size-6"></span>
                        </div>
                    </div>
                    <div class="stat-value mb-1">Revenue</div>
                    <div class="stat-title">$45,000 of $100,000</div>
                    <div
                        class="progress bg-warning/10 h-2"
                        role="progressbar"
                        aria-label="Revenue Progressbar"
                        aria-valuenow="45"
                        aria-valuemin="0"
                        aria-valuemax="100"
                    >
                        <div class="progress-bar progress-warning w-2/5"></div>
                    </div>
                </div>
            </div>

            <div class="stats max-sm:w-full">
                <div class="stat">
                    <div class="avatar placeholder">
                        <div class="bg-error/20 text-error size-10 rounded-full">
                            <span class="icon-[tabler--credit-card] size-6"></span>
                        </div>
                    </div>
                    <div class="stat-value mb-1">Invoice</div>
                    <div class="stat-title">$18,200 of $25,000</div>
                    <div
                        class="progress bg-error/10 h-2"
                        role="progressbar"
                        aria-label="Invoice Progressbar"
                        aria-valuenow="73"
                        aria-valuemin="0"
                        aria-valuemax="100"
                    >
                        <div class="progress-bar progress-error w-9/12"></div>
                    </div>
                </div>
            </div>

            <div class="stats">
                <div class="stat">
                    <div class="avatar placeholder">
                        <div class="bg-info/20 text-info size-10 rounded-full">
                            <span class="icon-[tabler--truck] size-6"></span>
                        </div>
                    </div>
                    <div class="stat-value mb-1">Shipments</div>
                    <div class="stat-title">10,450 of 12,000 shipments</div>
                    <div
                        class="progress bg-info/10 h-2"
                        role="progressbar"
                        aria-label="Invoice Progressbar"
                        aria-valuenow="73"
                        aria-valuemin="0"
                        aria-valuemax="100"
                    >
                        <div class="progress-bar progress-info w-11/12"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
