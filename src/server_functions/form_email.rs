use leptos::*;
use std::env;
    
#[server(SendEmail, "/api")]
pub async fn send_email(name: String, email: String, message: String) -> Result<(), ServerFnError> {

    use lettre::{
        message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
        AsyncTransport, Message, Tokio1Executor,
    };
    let body = String::from(format!("Message:\n From: {}({}) \n {}", name, email, message));
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
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

