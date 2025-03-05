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
pub mod league_match_list;
pub mod league_panel;
pub mod leagues_panel;

use crate::app::{
    AppRoute,
    matches::MatchesRoute,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum LeagueRoute {
    #[default]
    #[target(index)]
    Details,
    Create,
    Matches {
        #[target(nested, default)]
        action: MatchesManagementRoute,
    },
    Match {
        match_id: Uuid,
    },
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesManagementRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
}

impl LeagueRoute {
    pub fn mapper_matches(league_id: Uuid) -> Mapper<LeagueRoute, MatchesRoute> {
        let downwards = move |page| {
            match page {
                LeagueRoute::Matches {
                    ..
                } => {
                    Some(MatchesRoute::Matches {
                        league_id,
                    })
                }
                _ => None,
            }
        };
        let upwards = move |_| {
            LeagueRoute::Matches {
                action: MatchesManagementRoute::Index,
            }
        };

        Mapper::new(downwards, upwards)
    }

    pub fn mapper_match_create(league_id: Uuid) -> Mapper<LeagueRoute, MatchesRoute> {
        let downwards = move |page| {
            match page {
                LeagueRoute::Matches {
                    action: MatchesManagementRoute::Create,
                } => {
                    Some(MatchesRoute::Create {
                        league_id,
                    })
                }
                _ => None,
            }
        };
        let upwards = move |matches_route| {
            match matches_route {
                MatchesRoute::Create {
                    ..
                } => {
                    LeagueRoute::Matches {
                        action: MatchesManagementRoute::Create,
                    }
                }
                _ => {
                    LeagueRoute::Matches {
                        action: MatchesManagementRoute::Index,
                    }
                }
            }
        };

        Mapper::new(downwards, upwards)
    }
}

pub fn leagues_nav_menu() -> Html {
    html_nested! {
        <>
            <NavRouterItem<AppRoute>
                to={AppRoute::Leagues { action: crate::app::LeaguesManagementRoute::Index}}
            >
                { "Leagues" }
            </NavRouterItem<AppRoute>>
        </>
    }
}

#[function_component(Index)]
fn leagues_index() -> Html {
    // lsdkjlfdsjk
    html!({ "Leagues Index Component." })
}
