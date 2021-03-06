pub mod morphology {
    use serde::{Serialize, Deserialize};
    use reqwest::Error;
    use std::env;

    #[derive(Deserialize)]
    struct Response {
        word_list: Vec<Vec<Vec<String>>>,
    }

    #[derive(Serialize)]
    struct Request {
        app_id: String,
        sentence: String,
        info_filter: String,
    }

    #[derive(Debug)]
    pub struct Morpheme {
        word: String,
        class: String,
    }

    const REQUEST_URL: &str = "https://labs.goo.ne.jp/api/morph";

    pub async fn get_parts(phrase: String) -> Result<Vec<Morpheme>, Error> {
        let api_id = env::var("JAP_TOKEN").unwrap_or("".to_string());
        let params = Request {
            app_id: api_id,
            sentence: phrase,
            info_filter: String::from("form|pos")
        };

        let client = reqwest::Client::new();
        let response = client
            .post(REQUEST_URL)
            .form(&params)
            .send()
            .await?;

        let response: Response = response.json().await?;
        let result: Vec<Morpheme> = response.word_list
            .into_iter()
            .flatten()
            .map(|item| Morpheme { word: item[0].clone(), class: item[1].clone()})
            .collect();
            
        Ok(result)
    }
}