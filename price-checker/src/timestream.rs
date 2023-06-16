
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use reqwest::blocking::Client;
#[allow(unused_imports)]
use reqwest::header::{HeaderValue, AUTHORIZATION};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use chrono::{Utc, DateTime};
#[allow(unused_imports)]
use chrono::format::strftime::StrftimeItems;
use serde_json::Value;
use std::io::Error;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Endpoints {
    pub Address: String,
    pub CachePeriodInMinutes: f32,
}

impl Endpoints {
    fn describe_endpoints() -> Result<Endpoints, reqwest::Error> {
        let client = Client::new();
        let auth = Authentication::get().expect("Get auth file");
        let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        // This part makes me really miss Perl :(
        let auth_string = "AWS4-HMAC-SHA256 Credential=".to_owned() + &auth.access_key + "/" + &timestamp + "/timestream/aws4_request";
        let auth_header = (AUTHORIZATION, HeaderValue::from_str(&auth_string).expect("Make auth header string"));
        let response = client.get("https://timestream.us-east-1.amazonaws.com")
            .header(auth_header.0, auth_header.1)
            .send()?;
        
        let endpoints: Endpoints = response.json()?;
        return Ok(endpoints)
    }
}


#[derive(Deserialize, Debug)]
struct Authentication {
    access_key: String,
    secret_key: String,
}

impl Authentication {
    fn get() -> Result<Authentication, Error> {
        use std::fs::File;
        use std::io::Read;

        let mut auth_file = File::open("/.aws/creds.json")?;
        let mut file_contents = String::new();
        auth_file.read_to_string(&mut file_contents)?;
        let auth: Value = serde_json::from_str(&file_contents)?;

        let access_key = auth["access_key"].to_string();
        let secret_key = auth["secret_key"].to_string();
        
        Ok(Authentication {
            access_key,
            secret_key,
        })
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NewRecord {

}

impl NewRecord {

}
