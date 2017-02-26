use std::collections::HashMap;
use serde_json;
use serde_json::{Map, Value};
use reqwest::{Client as HttpClient, RequestBuilder};
use reqwest::Response;
use reqwest::header::Authorization;
use std::io::Read;

struct Client<'a> {
    client_access_token: &'a str,
    api_lang: &'a str,
    session_id: &'a str,
}

impl<'a> Client<'a> {
    fn new(client_access_token: &'a str,
           api_lang: &'a str,
           session_id: &'a str)
           -> Client<'a> {
        Client {
            client_access_token: client_access_token,
            api_lang: api_lang,
            session_id: session_id,
        }
    }

    fn text_request(&self,
                        query: &'a str,
                        mut options: HashMap<&'a str, &'a str>)
                        -> Result<Map<String, Value>, Map<String, Value>> {
        options.insert("query", query);
        options.insert("lang", self.api_lang);
        options.insert("sessionId", self.session_id);

        let mut auth = "Bearer ".to_string();
        auth.push_str(&self.client_access_token);

        let client = HttpClient::new().unwrap();
        let mut resp = client.post("https://api.api.ai/v1/query?v=20150910")
            .json(&options)
            .header(Authorization(auth))
            .send()
            .unwrap();

        let body: Map<String, Value> = resp.json().unwrap();

        if resp.status().is_success() {
            Ok(body)
        } else {
            Err(body)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_text_request() {
        let key = "TOKEN";
        let token = match env::var(key) {
            Ok(val) => val,
            Err(e) => {
                println!("Could not fetch {}: {}", key, e);
                assert!(false);
                "".to_string()
            }
        };

        // let token : &str = &token.clone();

        let client = Client::new(&token, "de", "12");
        let mut hash = HashMap::new();
        hash.insert("timezone", "Europe/Paris");

        let response = client.text_request("Hallo", hash);
        let map      = response.unwrap();

        assert!(map.contains_key("id"));
        assert!(map.contains_key("timestamp"));
        assert!(map.contains_key("lang"));
        assert!(map.contains_key("result"));
        assert!(map.contains_key("sessionId"));

        let result = map.get("result").unwrap().as_object().unwrap();
        assert!(result.contains_key("action"));
        assert!(result.contains_key("fulfillment"));
        assert!(result.contains_key("contexts"));

        let fulfillment = result.get("fulfillment").unwrap().as_object().unwrap();
        assert!(fulfillment.contains_key("speech"));
        assert!(fulfillment.contains_key("messages"));
    }
}