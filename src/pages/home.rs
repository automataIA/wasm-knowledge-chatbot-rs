use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="min-h-screen bg-base-100">
                <div class="hero min-h-screen bg-base-200">
                    <div class="hero-content text-center">
                        <div class="max-w-md">
                            <picture>
                                <source
                                    srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                                    media="(prefers-color-scheme: dark)"
                                />
                                <img
                                    src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                                    alt="Leptos Logo"
                                    height="200"
                                    width="400"
                                    class="mb-8"
                                />
                            </picture>

                            <h1 class="text-5xl font-bold text-primary">"Welcome to Leptos"</h1>
                            <p class="py-6 text-base-content">"Testing DaisyUI themes with Tailwind CSS"</p>

                            <div class="flex flex-col gap-4">
                                <div class="card bg-base-100 shadow-xl">
                                    <div class="card-body">
                                        <h2 class="card-title">"DaisyUI Components Test"</h2>
                                        <div class="flex gap-2 flex-wrap justify-center">
                                            <button class="btn btn-primary">"Primary"</button>
                                            <button class="btn btn-secondary">"Secondary"</button>
                                            <button class="btn btn-accent">"Accent"</button>
                                            <button class="btn btn-ghost">"Ghost"</button>
                                        </div>
                                    </div>
                                </div>

                                <div class="buttons flex gap-4 justify-center">
                                    <Button />
                                    <Button increment=5 />
                                </div>

                                <div class="alert alert-info">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <span>"DaisyUI theme is working!"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
