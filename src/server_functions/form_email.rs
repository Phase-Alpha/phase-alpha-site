use leptos::prelude::*;

/// Sends the contact form.
///
/// `turnstile_token` is populated by the Turnstile widget and `company_website`
/// is a honeypot: it is hidden from real visitors, so anything that arrives
/// with it filled in is a bot.
///
/// Both are optional so that a submission which omits them (a direct POST, or a
/// browser where the widget could not load) fails our own checks with a useful
/// message rather than failing to deserialize.
#[server(SendEmail, "/api")]
pub async fn send_email(
    name: String,
    email: String,
    message: String,
    turnstile_token: Option<String>,
    company_website: Option<String>,
) -> Result<String, ServerFnError> {
    use crate::server_functions::turnstile::verify_turnstile;
    use dotenv::dotenv;
    use leptos::logging::log;
    use lettre::{
        message::header::ContentType, message::Mailbox,
        transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
        Tokio1Executor,
    };
    use std::env;

    dotenv().ok();

    // Honeypot first: it costs nothing and filters the naive bots before we
    // spend a network round trip verifying a token. Report success so the bot
    // has no signal that it was caught.
    if company_website.is_some_and(|field| !field.trim().is_empty()) {
        log!("contact form: discarding submission that tripped the honeypot");
        return Ok(String::from("Message sent!"));
    }

    verify_turnstile(turnstile_token.as_deref().unwrap_or_default()).await?;

    let body = String::from(format!(
        "Message:\n From: {}({}) \n {}",
        name, email, message
    ));
    let email = Message::builder()
        .from(
            env::var("FROM_EMAIL")
                .expect("FROM_EMAIL env variable should be set")
                .parse::<Mailbox>()
                .unwrap(),
        )
        .to(env::var("TO_EMAIL")
            .expect("TO_EMAIL env variable should be set")
            .parse::<Mailbox>()
            .unwrap())
        .subject("Website Form Contact")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();
    let smtp_username: String =
        env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable should be set");
    let smtp_password: String =
        env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable should be set");
    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => Ok(String::from("Message sent!")),
        Err(_) => Err(ServerFnError::ServerError(
            "Could not send message :(".to_string(),
        )),
    }
}
