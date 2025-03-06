use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use shared_types::response::Match;
use uuid::Uuid;
use yew_nested_router::prelude::*;

pub mod match_details_panel;
pub mod matches_create_panel;
pub mod matches_list_panel;
pub mod matches_panel;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum MatchRoute {
    #[default]
    #[target(index)]
    Details,
}

pub async fn fetch_matches(league_id: Uuid) -> Result<Vec<Match>> {
    let response = Request::get(&format!("/api/league/{league_id}/match")).send().await?;
    let matches = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch matches for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(matches)
}

pub async fn fetch_match(league_id: Uuid, match_id: Uuid) -> Result<Match> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}")).send().await?;
    let match_object = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch matches for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(match_object)
}
