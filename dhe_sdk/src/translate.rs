use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

use crate::language::Language;

#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("failed to complete request with error: {0}")]
    FailedToRequest(String),
    #[error("failed to parse request with error: {0}")]
    FailedToParseRequest(String),
}

/// Translation of text through google translator.
pub async fn translate(text: &str, from: Language, to: Language) -> Result<String, TranslateError> {
    let mut url = Url::parse("https://translate.google.com/m").unwrap();
    url.query_pairs_mut()
        .append_pair("sl", &from.to_string())
        .append_pair("tl", &to.to_string())
        .append_pair("q", text);

    let html = reqwest::get(url)
        .await
        .map_err(|err| TranslateError::FailedToRequest(err.to_string()))?
        .text()
        .await
        .map_err(|err| TranslateError::FailedToParseRequest(err.to_string()))?;

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse(".result-container").unwrap();
    let element = fragment.select(&selector).next().ok_or_else(|| {
        TranslateError::FailedToParseRequest(
            "translation not found as a result of the query".to_string(),
        )
    })?;
    Ok(element.inner_html())
}
