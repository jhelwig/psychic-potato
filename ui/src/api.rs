use gloo_net::http::Request;
use log::error;
use serde::{
    Deserialize,
    Serialize,
};
use yew::prelude::*;

pub async fn perform_api_operation<O, R>(
    api_endpoint: String,
    operation: O,
    maybe_item_setter: UseStateSetter<Option<Result<R, String>>>,
) where
    O: Clone + Serialize,
    R: for<'de> Deserialize<'de>,
{
    let response = match Request::post(&api_endpoint).json(&operation) {
        Ok(req) => req.send().await,
        Err(error) => {
            error!("Unable to set request body: {}", error);
            maybe_item_setter.set(Some(Err(error.to_string())));
            return;
        }
    };
    match response {
        Ok(resp) => {
            if resp.ok() {
                match resp.json().await {
                    Ok(result) => {
                        maybe_item_setter.set(Some(Ok(result)));
                    }
                    Err(error) => {
                        error!("Unable to parse response: {error}");
                        maybe_item_setter.set(Some(Err(error.to_string())));
                    }
                }
            } else {
                let error_body = match resp.text().await {
                    Ok(text) => text,
                    Err(error) => error.to_string(),
                };
                error!("Operation failed ({}): {error_body}", resp.status());
                maybe_item_setter.set(Some(Err(format!(
                    "{} {}: {error_body}",
                    resp.status(),
                    resp.status_text(),
                ))));
            }
        }
        Err(error) => {
            error!("Error sending request: {error}");
            maybe_item_setter.set(Some(Err(error.to_string())));
        }
    }
}
