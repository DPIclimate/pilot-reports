use yup_oauth2::AccessToken;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageListShort {
    #[serde(default)] // Denotes optional field below
    pub messages: Vec<MessageShort>,
    pub result_size_estimate: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageShort {
    pub id: String,
    pub thread_id: String,
}


#[tokio::main]
pub async fn list(access: &AccessToken, email: &String) -> Result<MessageListShort, Box<dyn Error>> {
    // Gets a list of emails in the inbox
    // This returns a short response only containing email id's and thread id's
    // Email id's are used to identify emails and can be used to get the full message
    // in a get request

    let url = format!("https://gmail.googleapis.com/gmail/v1/users/{}/messages", email);

    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .bearer_auth(access.as_str())
        .send()
        .await?
        .json::<MessageListShort>()
        .await?;

    Ok(res)
}

#[tokio::main]
pub async fn clear_inbox(access: &AccessToken, 
    email: &String, msg_list: &MessageListShort) -> Result<(), Box<dyn Error>> {
    // Move any messages in the inbox to trash
    // This method is used to clear the gmail inbox BEFORE requesting new data
    // This way when the list() method is called it only gets emails sent in the current batch

    println!("Clearing inbox to trash");

    for msg in &msg_list.messages{
        let url = format!("https://gmail.googleapis.com/gmail/v1/users/{}/messages/{}/trash", 
            email, msg.id);

        let client = reqwest::Client::new();

        let res = client
            .post(url)
            .bearer_auth(access.as_str())
            .header("Content-Type", "application/json")
            .header("Content-Length", "0")
            .send()
            .await?;

        println!("Status: {} for message id: {}", res.status(), msg.id);
    }

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub thread_id: String,
    pub label_ids: Vec<String>,
    pub snippet: String, // This contains the message (body of the email)
    pub payload: Payload,
    pub size_estimate: i64,
    pub history_id: String,
    pub internal_date: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub part_id: String,
    pub mime_type: String,
    pub filename: String,
    //pub headers: Vec<Header>,
    //pub body: Body,
}

#[tokio::main]
pub async fn read_message(access: &AccessToken, email: &String, 
    msg_id: &String) -> Result<Message, Box<dyn Error>> {
    // Get message in inbox using message id
    // Returns a Message

    let url = format!("https://gmail.googleapis.com/gmail/v1/users/{}/messages/{}", 
        email, msg_id);

    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .bearer_auth(access.as_str())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Message>()
        .await?;

    Ok(res)
}

pub fn extract_url(message: &Message) -> String {
    // Extract url from message body (snippet)
    // TODO: Can this be improved?
    // Returns a string containing the url
    let split_body = message.snippet.split(" ");

    for item in split_body{
        if item.contains(".csv"){
            return item.to_string();
        }
    }
    return String::from("")
}



