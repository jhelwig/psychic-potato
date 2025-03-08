use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::League;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::{
    Target,
    prelude::*,
};

pub mod league_create_panel;
pub mod league_details_panel;
pub mod league_list;
pub mod league_panel;
pub mod leagues_panel;

use crate::app::{
    AppRoute,
    classes::{
        ClassRoute,
        ClassesRoute,
    },
    matches::{
        MatchRoute,
        MatchesRoute,
    },
    shooters::ShootersRoute,
};

#[remain::sorted]
#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum LeaguesRoute {
    Create,
    #[default]
    #[target(index)]
    Index,
}

#[remain::sorted]
#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum LeagueRoute {
    Class {
        class_id: Uuid,
        #[target(nested, default)]
        page:     ClassRoute,
    },
    Classes(ClassesRoute),
    #[default]
    #[target(index)]
    Details,
    Match {
        match_id: Uuid,
        #[target(nested, default)]
        page:     MatchRoute,
    },
    Matches(MatchesRoute),
    Shooters(ShootersRoute),
}

impl LeagueRoute {
    pub fn mapper_match(match_id: Uuid) -> Mapper<LeagueRoute, MatchRoute> {
        let downwards = move |page| {
            match page {
                LeagueRoute::Match {
                    page,
                    ..
                } => Some(page),
                _ => None,
            }
        };
        let upwards = move |page| {
            LeagueRoute::Match {
                match_id,
                page,
            }
        };

        Mapper::new(downwards, upwards)
    }

    pub fn mapper_class(class_id: Uuid) -> Mapper<LeagueRoute, ClassRoute> {
        let downwards = move |page| {
            match page {
                LeagueRoute::Class {
                    page,
                    ..
                } => Some(page),
                _ => None,
            }
        };
        let upwards = move |page| {
            LeagueRoute::Class {
                class_id,
                page,
            }
        };

        Mapper::new(downwards, upwards)
    }
}

pub fn leagues_nav_menu() -> Html {
    html_nested! {
        <>
            <NavRouterItem<AppRoute> to={AppRoute::Leagues(LeaguesRoute::Index)}>
                { "Leagues" }
            </NavRouterItem<AppRoute>>
        </>
    }
}

pub async fn fetch_league(league_id: Uuid) -> Result<League> {
    let response = Request::get(&format!("/api/league/{league_id}")).send().await?;
    let league: League = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch league: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(league)
}
