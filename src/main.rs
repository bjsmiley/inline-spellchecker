
fn main() -> Result<(),srv::Error>{
    
    let utterance = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    let corrected = srv::spellcheck(utterance)?;

    println!("{:?}", corrected);

    Ok(())
}

mod core {

    fn exec() {

    }
}


mod srv {
    use reqwest::{blocking::{ClientBuilder}, Method};


    const BING_URI: &str = "https://api.bing.microsoft.com/v7.0/spellcheck";
    const BING_KEY: &str = "";
    const MARKET_PARAM: &str = "en-US";
    const MODE_PARAM: &str = "spell";

    #[derive(Debug, serde::Deserialize)]
    pub struct BingResponse {
        pub errors: Option<String>,
        pub flagged_tokens: Vec<Token>
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Token {
        pub offset: usize,
        pub suggestions: Vec<Suggestion>
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Suggestion {
        pub suggestion: String,
        pub score: u32
    }

    pub fn spellcheck(text: String) -> Result<String,Error> {

        let client = ClientBuilder::new().build().map_err(|_| Error::Init)?;

        let result = client.request(Method::POST, BING_URI)
            .query(&[("text", text), ("mkt", MARKET_PARAM.to_owned()), ("mode", MODE_PARAM.to_owned())])
            .header("Ocp-Apim-Subscription-Key", BING_KEY)
            .header("content-length", 0)
            .send();

        let res: BingResponse = result.map_err(|e| {
            if e.is_status() {
                return Error::Status(e.status().unwrap().as_u16())
            }
            Error::Other(e)
        })?.json().unwrap();

        if let Some(e) =  res.errors {
            return Err(Error::Bing(e));
        }
        if res.flagged_tokens.len() == 0 {
            return Ok(text)
        }
        let words: Vec<&str> = text.split_whitespace().collect();
        for w in res.flagged_tokens.into_iter().filter(|x| x.suggestions.len() > 0) {
            words[w.offset] = w.suggestions[0].suggestion.clone();
        }

        Ok(words.join(" "))
    }

    #[derive(Debug)]
    pub enum Error {
        Init, // a TLS backend cannot be initialized, or the resolver cannot load the system configuration.
        Status(u16),
        Bing(String),
        Other(reqwest::Error)
    }
}
