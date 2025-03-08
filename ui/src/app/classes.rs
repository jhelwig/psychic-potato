use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use shared_types::response::Class;
use uuid::Uuid;
use yew_nested_router::prelude::*;

pub mod class_create_panel;
pub mod class_details_panel;
pub mod class_list_panel;
pub mod classes_panel;

#[remain::sorted]
#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum ClassRoute {
    #[default]
    #[target(index)]
    Details,
}

#[remain::sorted]
#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum ClassesRoute {
    Create,
    #[default]
    #[target(index)]
    Index,
}

pub async fn fetch_classes(league_id: Uuid) -> Result<Vec<Class>> {
    let response = Request::get(&format!("/api/league/{league_id}/class")).send().await?;
    let classes = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch classes for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(classes)
}

pub async fn fetch_class(league_id: Uuid, class_id: Uuid) -> Result<Class> {
    let response =
        Request::get(&format!("/api/league/{league_id}/class/{class_id}")).send().await?;
    let class = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch class {class_id} for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(class)
}
