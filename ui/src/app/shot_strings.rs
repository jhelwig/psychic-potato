use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use shared_types::response::{
    ShotMarkerShot,
    ShotMarkerShotString,
};
use uuid::Uuid;
use yew_nested_router::prelude::*;

pub mod shot_string_list_panel;
pub mod shot_string_panel;
pub mod shot_string_svg_panel;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum ShotStringsRoute {
    #[default]
    #[target(index)]
    Index,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum ShotStringRoute {
    #[default]
    #[target(index)]
    Details,
}

pub async fn fetch_shot_strings(
    league_id: Uuid,
    match_id: Uuid,
) -> Result<Vec<ShotMarkerShotString>> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}/string")).send().await?;
    let shot_strings = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch shot strings for match {match_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(shot_strings)
}

pub async fn fetch_shot_string(
    league_id: Uuid,
    match_id: Uuid,
    string_id: Uuid,
) -> Result<ShotMarkerShotString> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}/string/{string_id}"))
            .send()
            .await?;
    let shot_string = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch shot string {string_id} for match {match_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(shot_string)
}

pub async fn fetch_shots_for_string(
    league_id: Uuid,
    match_id: Uuid,
    string_id: Uuid,
) -> Result<Vec<ShotMarkerShot>> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}/string/{string_id}/shot"))
            .send()
            .await?;
    let shots = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch shots for shot string {string_id} for match {match_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(shots)
}
