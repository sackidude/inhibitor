#[derive(Debug)]
pub enum Error {
    ParseError,
    VerificationError,
}

#[derive(serde::Deserialize)]
pub struct Account {
    username: String,
    tag: String,
    region: String,
}

impl Account {
    pub async fn from_form_res(urlencoded: &str) -> Result<Self, Error> {
        // Parse the account
        let mut account: Self =
            serde_urlencoded::from_str(urlencoded).map_err(|_| Error::ParseError)?;

        account.region = account.region.trim().to_ascii_uppercase();
        // Verify that the account actually exists.
        if !account.exists().await {
            return Err(Error::VerificationError);
        }
        Ok(account)
    }

    async fn exists(&self) -> bool {
        let client = reqwest::Client::new();

        let url = self.get_url();

        let res_status = client.get(url).send().await.unwrap().status();

        return res_status == reqwest::StatusCode::OK;
    }

    pub fn get_url(&self) -> String {
        format!(
            "https://www.leagueofgraphs.com/summoner/{}/{}-{}",
            self.region.to_ascii_lowercase(), // Really weird but ok.
            self.username,
            self.tag
        )
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_region(&self) -> &str {
        &self.region
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }
}
