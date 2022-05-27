use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};

//  login login
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenLoginResult {
    #[serde(default)]
    goto: Option<String>,
}

impl From<String> for TokenLoginResult {
    fn from(token: String) -> Self {
        Self { goto: Some(token) }
    }
}

impl TokenLoginResult {
    pub fn get_authorization_code(&self) -> crate::Result<String> {
        if let Some(ref g) = self.goto {
            let url = Url::parse(g.as_str())?;
            if let Some(q) = url.query() {
                let q = q.to_string();
                let param_array: Vec<_> = q.split("&").collect();
                for param in param_array {
                    let param = param.to_string();
                    let k_v_array: Vec<_> = param.split("=").collect();
                    let key_option = k_v_array.get(0);
                    let key = key_option.unwrap_or(&"");
                    if *key == "code" {
                        let value_option = k_v_array.get(1);
                        let value = value_option.unwrap_or(&"");
                        return Ok(String::from(*value));
                    }
                }
            }
        }
        Err(anyhow!("get goto result error."))
    }
}
