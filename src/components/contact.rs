use crate::components::layout::Layout;
use crate::server_functions::form_email::*;
use crate::server_functions::turnstile::{CONTACT_FORM_ACTION, RESPONSE_FIELD_NAME};
use leptos::prelude::*;

/// Name of the honeypot input. Must match the `company_website` parameter of
/// [`send_email`].
const HONEYPOT_FIELD: &str = "company_website";

#[cfg(feature = "hydrate")]
mod turnstile_js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        // The sitekey is deliberately absent here: turnstile.js reads it from
        // the meta tag rendered by `shell`, so it stays a runtime setting.
        #[wasm_bindgen(js_namespace = window, js_name = mountTurnstile)]
        pub fn mount_turnstile(action: &str, field_name: &str);

        #[wasm_bindgen(js_namespace = window, js_name = resetTurnstile)]
        pub fn reset_turnstile();
    }
}

#[cfg(feature = "hydrate")]
use turnstile_js::{mount_turnstile, reset_turnstile};

// The widget is a browser-only concern; these keep the server build compiling.
#[cfg(not(feature = "hydrate"))]
fn mount_turnstile(_action: &str, _field_name: &str) {}

#[cfg(not(feature = "hydrate"))]
fn reset_turnstile() {}

#[component]
pub fn Contact() -> impl IntoView {
    let send_email = ServerAction::<SendEmail>::new();
    let value = send_email.value();
    let is_pending = send_email.pending();

    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());

    let is_valid = move || {
        !name().is_empty() && !email().is_empty() && email().contains('@') && !message().is_empty()
    };

    // Render the widget once the form is on the page. Doing this from an effect
    // rather than relying on Turnstile's automatic scan means it also appears
    // when the visitor arrives here through the router.
    Effect::new(move |_| {
        mount_turnstile(CONTACT_FORM_ACTION, RESPONSE_FIELD_NAME);
    });

    // A token is consumed by the submission that used it, so without this a
    // second attempt (after a mail failure, say) would be rejected as a
    // duplicate.
    Effect::new(move |_| {
        if value.with(Option::is_some) {
            reset_turnstile();
        }
    });

    let status_view = move || {
        value.get().map(|result| match result {
            Ok(msg) => view! { <p class="form-status form-status--ok">{msg}</p> }.into_any(),
            // Surface the server's own wording so a failed challenge reads
            // differently from a mail failure.
            Err(ServerFnError::ServerError(reason)) => {
                view! { <p class="form-status form-status--error">{reason}</p> }.into_any()
            }
            Err(_) => view! {
                <p class="form-status form-status--error">"Could not send message."</p>
            }
            .into_any(),
        })
    };

    view! {
        <Layout buffer="*get-in-touch*" mode="(Form)">
            <section class="section">
                <span class="eyebrow">";; ~/phase-alpha/contact"</span>
                <h1>"Get in touch"</h1>
                <p class="hero__tagline">
                    "Have a project in mind? Tell us about it and we will get back to you."
                </p>

                <ActionForm action=send_email attr:class="form">
                    <div class="field">
                        <label class="field__label" for="contact-name">";; name"</label>
                        <input
                            class="field__input"
                            id="contact-name"
                            type="text"
                            name="name"
                            autocomplete="name"
                            required
                            prop:value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>

                    <div class="field">
                        <label class="field__label" for="contact-email">";; email"</label>
                        <input
                            class="field__input"
                            id="contact-email"
                            type="email"
                            name="email"
                            autocomplete="email"
                            required
                            prop:value=email
                            on:input=move |ev| set_email(event_target_value(&ev))
                        />
                    </div>

                    <div class="field">
                        <label class="field__label" for="contact-message">";; message"</label>
                        <textarea
                            class="field__textarea"
                            id="contact-message"
                            name="message"
                            rows="5"
                            required
                            prop:value=message
                            on:input=move |ev| set_message(event_target_value(&ev))
                        />
                    </div>

                    // Honeypot. Positioned off screen rather than hidden with
                    // `display: none`, which bots have long since learned to
                    // skip, and taken out of the tab order and the
                    // accessibility tree so no real visitor can reach it.
                    <div class="hp-field" aria-hidden="true">
                        <label>
                            "Company website"
                            <input
                                type="text"
                                name=HONEYPOT_FIELD
                                tabindex="-1"
                                autocomplete="off"
                            />
                        </label>
                    </div>

                    // Turnstile renders into this and injects the token as a
                    // hidden input inside the form.
                    <div id="turnstile-widget" class="turnstile-widget"></div>

                    <div>
                        <button
                            type="submit"
                            class="btn btn--primary"
                            disabled=move || !is_valid() || is_pending()
                        >
                            "Send message"
                        </button>
                    </div>

                    <div aria-live="polite">
                        <Show when=is_pending>
                            <p class="form-status form-status--pending">"Sending…"</p>
                        </Show>
                        {status_view}
                    </div>
                </ActionForm>

                <ul class="contact-details">
                    <li>
                        <span class="contact-details__label">";; email"</span>
                        <a href="mailto:info@phasealpha.io">"info@phasealpha.io"</a>
                    </li>
                    <li>
                        <span class="contact-details__label">";; shop"</span>
                        <a
                            href="https://phaseatelier.etsy.com"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "phaseatelier.etsy.com"
                        </a>
                    </li>
                </ul>
            </section>
        </Layout>
    }
}
