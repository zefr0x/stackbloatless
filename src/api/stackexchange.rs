use reqwest::Url;
use serde::Deserialize;
use serde_json as json;

const API_ENDPOINT: &str = "https://api.stackexchange.com/2.3";

// API filters to just include fields we need.
const API_QUESTIONS_FILTER: &str =
    "!FQztYWDEEX-i2dYDg4I)KgNAG4qe0OakrOyxz.zkYgTMDp0*6TplKxYPRxMk-4.(QtKd(sF-";
// const API_SEARCH_FILTER: &str = "";

// When it's not big enough some results might be missing.
const API_SITE_PAGESIZE: &str = "100";

pub type Id = u32; // Since all operations are in strings not integers.
pub type Uri = String;
type Date = i64;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    display_name: String,
    link: Option<String>, // Url
    reputation: Option<u32>,
    user_id: Option<Id>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    body_markdown: Option<String>,
    comment_id: Id,
    creation_date: Date,
    owner: User,
    post_id: Id,
    score: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Answer {
    answer_id: Id,
    body_markdown: String,
    comment_count: u32,
    comments: Option<Vec<Comment>>,
    creation_date: Date,
    is_accepted: bool,
    last_activity_date: Date,
    owner: User,
    score: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Question {
    pub answer_count: u32,
    pub answers: Option<Vec<Answer>>,
    pub body_markdown: String,
    pub comment_count: u32,
    pub comments: Option<Vec<Comment>>,
    pub creation_date: Date,
    pub is_answered: bool,
    pub last_activity_date: Date,
    pub link: String, // Url
    pub owner: User,
    pub question_id: Id,
    pub score: i32,
    pub tags: Vec<String>,
    pub title: String,
    pub view_count: u32,
}

pub struct StackExchange {
    reqwest_client: reqwest::Client,
}

impl StackExchange {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::builder().gzip(true).build().unwrap(),
        }
    }

    pub async fn get_questions_from_uri(&self, uri: &str) -> Result<Vec<Question>, String> {
        // Accept uris of form: stackexchange://{site}/{questions ids}
        // For example: stackexchange://stackoverflow/123456;7891011;121314
        let uri = Url::parse(uri).unwrap();

        // TODO: Check if shame is stackexchange or not.
        // TODO: Check if questions ids are valid.

        self.get_questions(uri.domain().unwrap(), uri.path()).await
    }

    async fn get_questions(&self, site: &str, ids: &str) -> Result<Vec<Question>, String> {
        // Docs: https://api.stackexchange.com/docs/questions-by-ids
        //
        // `ids` are in form of a path with multiple ids separated by ;
        // For example: /123456;78910;111213
        let mut url = Url::parse(API_ENDPOINT)
            .unwrap()
            .join(&format!("questions{}", ids))
            .unwrap();

        url.set_query(Some(&format!(
            "site={site}&filter={API_QUESTIONS_FILTER}&pagesize={API_SITE_PAGESIZE}"
        )));

        let res = self.reqwest_client.get(url).send().await.unwrap();

        let value: json::Value = res.json().await.unwrap();

        // TODO: Handle backend errors
        if let Some(error_id) = value.get("error_id") {
            // value.get("error_name");
            // value.get("error_message");
            return Err(error_id.to_string());
        }

        Ok(json::from_value::<Vec<Question>>(value.get("items").unwrap().to_owned()).unwrap())
    }

    // TODO: Search function
    // fn search_questions() {
    //     // Docs: https://api.stackexchange.com/docs/search
    //     todo!();
    // }
}
