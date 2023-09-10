use clap::Parser;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use regex::Regex;

/// Send an email from the CLI
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// Name <email@address>
    #[arg(short, long)]
    sender: String,
    /// Name <email@address>
    #[arg(short, long)]
    receiver: String,
    /// [subject] Content
    #[arg(short, long)]
    content: String,
    /// Google app password (https://myaccount.google.com/apppasswords, you need 2FA enabled)
    #[arg(short, long)]
    password: String,
}

fn parse_2_from_regex(re: &str, content: &str, panic_msg: &str) -> (String, String) {
    let re = Regex::new(re).unwrap();
    let caps = match re.captures(content) {
        Some(caps) => caps,
        None => panic!("{}", panic_msg),
    };
    let first = match caps.get(1) {
        Some(first) => first.as_str().to_string(),
        None => panic!("{}", panic_msg),
    };
    let second = match caps.get(2) {
        Some(second) => second.as_str().to_string(),
        None => panic!("{}", panic_msg),
    };
    (first, second)
}

fn main() {
    let args = Args::parse();
    let (subject, content) =
        parse_2_from_regex(r"^\[(.*)\](.*)$", &args.content, "Invalid content format");
    let (sender_name, sender_email) =
        parse_2_from_regex(r"^(.*) <(.*)>$", &args.sender, "Invalid sender format");
    let (receiver_name, receiver_email) =
        parse_2_from_regex(r"^(.*) <(.*)>$", &args.receiver, "Invalid receiver format");

    let smtp_username = sender_email.clone();
    let smtp_password = args.password;

    let email = Message::builder()
        .from(
            format!("{} <{}>", sender_name, sender_email)
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", receiver_name, receiver_email)
            .parse()
            .unwrap())
        .header(ContentType::TEXT_PLAIN)
        .subject(subject)
        .body(content)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
