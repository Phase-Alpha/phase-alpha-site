use leptos::prelude::*;

#[server(SendEmail, "/api")]
pub async fn send_email(
    name: String,
    email: String,
    message: String,
) -> Result<String, ServerFnError> {
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
