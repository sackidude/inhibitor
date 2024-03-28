#[derive(Debug)]
pub enum Error {
    ParseError,
    VerificationError,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    Euw,
    Kr,
    Na,
    Eune,
}

#[derive(serde::Deserialize, Debug)]
pub struct Account {
    username: String,
    tag: String,
    region: Region,
}

impl Into<&str> for &Region {
    fn into(self) -> &'static str {
        match self {
            Region::Euw => "euw",
            Region::Kr => "kr",
            Region::Na => "na",
            Region::Eune => "eune",
        }
    }
}

impl Account {
    pub async fn from_form_res(form_str: &str) -> Result<Self, Error> {
        // Parse the account
        let low_case = form_str.to_ascii_lowercase();
        let account: Self = serde_urlencoded::from_str(&low_case).map_err(|_| Error::ParseError)?;

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
            Into::<&str>::into(&self.region), // Really weird but ok.
            self.username,
            self.tag
        )
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }
}

#[tokio::test]
async fn account_exist_test() {
    let account = Account::from_form_res("username=oscargus&tag=poop&region=euw")
        .await
        .unwrap();
}
