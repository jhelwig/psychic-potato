use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use shared_types::response::ShotMarkerExport;
use uuid::Uuid;
use yew_nested_router::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum SmExportsRoute {
    #[default]
    #[target(index)]
    Index,
    Upload,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum SmExportRoute {
    #[default]
    #[target(index)]
    Details,
}

pub mod sm_export_list_panel;
pub mod sm_export_panel;
pub mod sm_export_upload;
pub mod sm_exports_panel;

pub async fn fetch_sm_exports(league_id: Uuid, match_id: Uuid) -> Result<Vec<ShotMarkerExport>> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}/export")).send().await?;
    let exports = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch SM exports for match {match_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(exports)
}

pub async fn fetch_sm_export(
    league_id: Uuid,
    match_id: Uuid,
    export_id: Uuid,
) -> Result<ShotMarkerExport> {
    let response =
        Request::get(&format!("/api/league/{league_id}/match/{match_id}/export/{export_id}"))
            .send()
            .await?;
    let export = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch SM export {export_id} for match {match_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(export)
}
