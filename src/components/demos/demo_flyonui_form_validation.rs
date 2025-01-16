use leptos::prelude::*;

#[component]
pub fn DemoFlyonUIFormValidation() -> impl IntoView {
    view! {
        <div class="bg-base-100 w-full rounded-lg shadow">
            <h5 class="bg-base-300 rounded-t-lg p-4 text-base text-xl font-bold">JS Validation</h5>
            <div class="w-full p-4">
                <form class="needs-validation peer grid gap-y-4" novalidate=true>
                    <div class="w-full">
                        <h6 class="text-lg font-semibold">1. Account Details</h6>
                        <hr class="mb-4 mt-2" />
                    </div>
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                        <div>
                            <label class="label label-text" for="firstName">
                                First Name
                            </label>
                            <input
                                id="firstName"
                                type="text"
                                placeholder="John"
                                class="input"
                                required
                            />
                            <span class="error-message">Please enter your name.</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                        <div>
                            <label class="label label-text" for="lastName">
                                Last Name
                            </label>
                            <input
                                id="lastName"
                                type="text"
                                placeholder="Doe"
                                class="input"
                                required
                            />
                            <span class="error-message">Please enter your last name.</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                    </div>
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                        <div>
                            <label class="label label-text" for="userEmail">
                                Email
                            </label>
                            <input
                                id="userEmail"
                                type="email"
                                class="input"
                                placeholder="john@gmail.com"
                                aria-label="john@gmail.com"
                                required=""
                            />
                            <span class="error-message">Please enter a valid email</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                        <div>
                            <label class="label label-text" for="userPassword">
                                Password
                            </label>
                            <div class="input-group">
                                <input
                                    id="userPassword"
                                    type="password"
                                    class="input"
                                    placeholder="Enter password"
                                    required
                                />
                                <span class="input-group-text">
                                    <button
                                        type="button"
                                        data-toggle-password="{ \"target\": \"#userPassword\" }"
                                        class="block"
                                    >
                                        <span class="icon-[tabler--eye] text-base-content/80 password-active:block hidden size-4 flex-shrink-0"></span>
                                        <span class="icon-[tabler--eye-off] text-base-content/80 password-active:hidden block size-4 flex-shrink-0"></span>
                                    </button>
                                </span>
                            </div>
                            <span class="error-message">Please enter a valid password</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                    </div>
                    <div class="w-full">
                        <h6 class="text-lg font-semibold">2. Personal Info</h6>
                        <hr class="mb-4 mt-2" />
                    </div>
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                        <div>
                            <label class="label label-text" for="userProfile">
                                Profile Pic
                            </label>
                            <input id="userProfile" type="file" class="input" required />
                            <span class="error-message">Please select the file</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                        <div>
                            <label class="label label-text" for="jsPickr">
                                DOB
                            </label>
                            <input
                                id="jsPickr"
                                type="text"
                                class="input"
                                placeholder="YYYY-MM-DD"
                                id="jsPickr"
                                required
                            />
                            <span class="error-message">Please select your DOB</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                    </div>
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                        <div>
                            <label class="label label-text" for="userCountry">
                                Pick your Country
                            </label>
                            <select
                                class="select"
                                id="userCountry"
                                aria-label="Select Country"
                                required
                            >
                                <option value="">Select Country</option>
                                <option value="usa">USA</option>
                                <option value="uk">UK</option>
                                <option value="france">France</option>
                                <option value="australia">Australia</option>
                                <option value="spain">Spain</option>
                            </select>
                            <span class="error-message">Please select your country</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                        <div>
                            <div class="label label-text">Gender</div>
                            <div class="flex items-center gap-2">
                                <input
                                    type="radio"
                                    id="male"
                                    name="radio-0"
                                    class="radio radio-primary"
                                    required
                                />
                                <label class="label label-text text-base" for="male">
                                    Male
                                </label>
                            </div>
                            <div class="flex items-center gap-2">
                                <input
                                    type="radio"
                                    id="female"
                                    name="radio-0"
                                    class="radio radio-primary"
                                    required
                                />
                                <label class="label label-text text-base" for="female">
                                    Female
                                </label>
                            </div>
                            <span class="error-message">Please select your Gender</span>
                            <span class="success-message">Looks good!</span>
                        </div>
                    </div>
                    <div class="w-full">
                        <label class="label label-text" for="userBio">
                            Bio
                        </label>
                        <textarea
                            class="textarea min-h-20 resize-none"
                            id="userBio"
                            placeholder="Hello!!!"
                            required=""
                        ></textarea>
                        <span class="error-message">Please write few words</span>
                        <span class="success-message">Looks good!</span>
                    </div>
                    <div>
                        <div class="flex items-center gap-3">
                            <input
                                type="checkbox"
                                id="userSwitch"
                                class="switch switch-primary"
                                required
                            />
                            <label class="label text-base" for="userSwitch">
                                Send me related emails
                            </label>
                        </div>
                        <span class="error-message">Please select your preference</span>
                        <span class="success-message">Looks good!</span>
                    </div>
                    <div>
                        <div class="flex items-center gap-3">
                            <input
                                type="checkbox"
                                class="checkbox checkbox-primary"
                                id="userAgre"
                                required
                            />
                            <label class="label text-base" for="userAgre">
                                Agree to our terms and conditions
                            </label>
                        </div>
                        <span class="error-message">Please confirm our T&C</span>
                        <span class="success-message">Looks good!</span>
                    </div>
                    <div class="mt-4">
                        <button type="submit" name="submitButton" class="btn btn-primary">
                            Submit
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
