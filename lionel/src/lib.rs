use std::collections::HashMap;

pub struct Client {
    client: reqwest::Client,
    login_url: String,
}

impl Client {
    pub fn default() -> Client {
        let client = reqwest::Client::new();
        let login_url = "https://lionel2.kgv.edu.hk/login/index.php".to_string();
        Client { client, login_url }
    }

    pub fn init(client: Option<reqwest::Client>, login_url: Option<String>) -> Client {
        let client = match client {
            Some(client) => client,
            None => reqwest::Client::new(),
        };
        let login_url = match login_url {
            Some(login_url) => login_url,
            None => "https://lionel2.kgv.edu.hk/login/index.php".to_string(),
        };
        Client { client, login_url }
    }

    pub async fn get_cookies(&self, username: &str, password: &str) -> HashMap<String, String> {
        let params = [("username", username), ("password", password)];
        let res = self.client.post(&self.login_url)
            .form(&params)
            .send()
            .await
            .unwrap();

        let cookies = res.cookies().collect::<Vec<_>>();
        
        let mut vals = HashMap::new();
        for cookie in cookies.iter() {
            vals.insert(cookie.name().to_string(), cookie.value().to_string());
        }
        
        vals
    }
}
