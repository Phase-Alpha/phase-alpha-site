use leptos::*;

#[server(SendEmail, "/api")]
pub async fn send_email(name: String, email: String, message: String) -> Result<(), ServerFnError> {
    use dotenv::dotenv;
    use lettre::{
        message::header::ContentType, message::Mailbox,
        transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
        Tokio1Executor,
    };
    use std::env;

    dotenv().ok();

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
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new async year")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // Open a remote connection to gmail
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
