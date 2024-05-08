use serde::Serialize;
use crate::Error;
use crate::proto::{Command, CommandKind};

#[derive(Serialize, Debug)]
pub struct OAuth2Token {
    code: String,
}

#[derive(Serialize, Debug)]
pub struct OAuth2TokenRequest {
    client_id: String,
    prompt: String,
    redirect_url: String,
    scopes: Vec<String>,
}

impl crate::Discord {
    pub async fn get_oauth(&self, req: OAuth2TokenRequest) -> Result<String, Error> {
        let rx = self.send_rpc(CommandKind::GetOAuth2Token, req).await?;

        handle_response!(rx, Command::GetOAuth2Token(oauth) => {
            Ok(oauth.unwrap().code)
        });
    }
}