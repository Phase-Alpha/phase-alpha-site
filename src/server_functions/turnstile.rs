//! Cloudflare Turnstile integration.
//!
//! The constants below are deliberately not feature gated: the client uses them
//! to configure the widget and the server uses them to validate what the widget
//! produced. Keeping them in one place stops the two sides drifting apart.

/// Identifies the contact form widget. Siteverify echoes this back, which lets
/// us reject tokens that were minted by some other widget sharing our sitekey.
///
/// Cloudflare validates this against `/^[a-z0-9_-]{0,32}$/i`.
pub const CONTACT_FORM_ACTION: &str = "contact-form";

/// Name of the hidden input Turnstile injects into the contact form.
///
/// Turnstile defaults to `cf-turnstile-response`, which is not a valid Rust
/// identifier and therefore cannot be a server function parameter. Overriding
/// it via the widget's `response-field-name` option lets the token ride along
/// with the rest of the form.
///
/// This MUST match the `turnstile_token` parameter of
/// [`crate::server_functions::form_email::send_email`].
pub const RESPONSE_FIELD_NAME: &str = "turnstile_token";

#[cfg(feature = "ssr")]
const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

/// Cloudflare caps tokens at 2048 characters.
#[cfg(feature = "ssr")]
const MAX_TOKEN_LEN: usize = 2048;

#[cfg(feature = "ssr")]
#[derive(serde::Deserialize)]
struct SiteVerifyResponse {
    success: bool,
    #[serde(default)]
    action: Option<String>,
    #[serde(rename = "error-codes", default)]
    error_codes: Vec<String>,
}

/// Validates a Turnstile token against Cloudflare's siteverify endpoint.
///
/// Tokens are single use and expire five minutes after they are issued, so a
/// replayed or stale token comes back as `timeout-or-duplicate`.
#[cfg(feature = "ssr")]
pub async fn verify_turnstile(token: &str) -> Result<(), leptos::prelude::ServerFnError> {
    use leptos::logging::log;
    use leptos::prelude::ServerFnError;

    // Deliberately vague: a bot should not learn which check it tripped.
    let rejected = || {
        ServerFnError::ServerError(
            "Could not verify that you are human. Please refresh and try again.".to_string(),
        )
    };
    let unavailable = || {
        ServerFnError::ServerError("Could not send message, please try again shortly.".to_string())
    };

    // Cheap local checks first so obvious junk never costs us a round trip.
    if token.is_empty() || token.len() > MAX_TOKEN_LEN {
        log!("turnstile: rejecting token of length {}", token.len());
        return Err(rejected());
    }

    let secret = std::env::var("TURNSTILE_SECRET_KEY").map_err(|_| {
        log!("turnstile: TURNSTILE_SECRET_KEY is not set");
        unavailable()
    })?;

    let response = reqwest::Client::new()
        .post(SITEVERIFY_URL)
        .form(&[("secret", secret.as_str()), ("response", token)])
        .send()
        .await
        .map_err(|e| {
            log!("turnstile: siteverify request failed: {e}");
            unavailable()
        })?
        .json::<SiteVerifyResponse>()
        .await
        .map_err(|e| {
            log!("turnstile: could not decode siteverify response: {e}");
            unavailable()
        })?;

    if !response.success {
        log!("turnstile: token rejected, error codes: {:?}", response.error_codes);
        return Err(rejected());
    }

    // A valid token from a different widget is still not a valid contact form
    // submission.
    if response.action.as_deref() != Some(CONTACT_FORM_ACTION) {
        log!("turnstile: unexpected action {:?}", response.action);
        return Err(rejected());
    }

    Ok(())
}
