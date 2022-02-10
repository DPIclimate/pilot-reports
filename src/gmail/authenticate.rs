use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod, AccessToken};

#[tokio::main]
pub async fn run() -> AccessToken {
    // This function authenticates with gmail via OAuth2
    // The InstalledFlowAuthenticator handles the refreshing of tokens
    // Client information (downloaded from google console) is held in the
    // clientsecret.json file (in root)
    // Once authenticated the auth details are stored in tokencache.json
    // This contains the auth token (used in requests "Bearer: auth-token")
    // and the refresh token (used by InstalledFlowAuthenticator)

    let secret = yup_oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("Requires valid clientsecret.json file");

    let auth = InstalledFlowAuthenticator::builder(secret, 
        InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

    // This scope is the master scope for gmail,
    // It has rights to create, read, update, send and delete emails
    let scopes = &["https://mail.google.com/"];

    match auth.token(scopes).await {
        Ok(token) => token,
        Err(e) => panic!("{}", e)
    }
}



