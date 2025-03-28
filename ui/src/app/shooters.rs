use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use shared_types::response::Shooter;
use uuid::Uuid;
use yew_nested_router::prelude::*;

pub mod shooter_autocomplete;
pub mod shooters_create_panel;
pub mod shooters_detail_panel;
pub mod shooters_list_panel;
pub mod shooters_panel;

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum ShootersRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
    Detail {
        shooter_id: Uuid,
    },
}

pub async fn fetch_shooter_suggestions(query: &str) -> Result<Vec<Shooter>> {
    let response =
        Request::get("/api/shooter/suggestions").query([("query", query)]).send().await?;
    let suggestions = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch shooter suggestions: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(suggestions)
}
