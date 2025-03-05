use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::{
    Target,
    prelude::*,
};

pub mod create_league_panel;
pub mod league_details_panel;
pub mod league_list;
pub mod league_panel;
pub mod leagues_panel;

use crate::app::{
    AppRoute,
    matches::{
        MatchRoute,
        MatchesRoute,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum LeaguesRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum LeagueRoute {
    #[default]
    #[target(index)]
    Details,
    Matches(MatchesRoute),
    Match {
        match_id: Uuid,
        #[target(nested, default)]
        page:     MatchRoute,
    },
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
