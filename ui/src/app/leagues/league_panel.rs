use log::info;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    AppRoute,
    leagues::{
        LeagueRoute,
        league_details_panel::LeagueDetailsPanel,
    },
    matches::{
        match_details_panel::MatchDetailsPanel,
        matches_panel::MatchesPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeaguePanelProps {
    pub league_id: Uuid,
}

#[function_component(LeaguePanel)]
pub fn league_panel(props: &LeaguePanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <>
            <Scope<AppRoute,LeagueRoute> mapper={move |_| { AppRoute::mapper_league(league_id)}}>
                <RouterSwitch<LeagueRoute>
                    render={move |target| { switch_league_panel(league_id, target)}}
                />
            </Scope<AppRoute,LeagueRoute>>
        </>
    }
}

pub fn switch_league_panel(league_id: Uuid, target: LeagueRoute) -> Html {
    let route = match target {
        LeagueRoute::Details => {
            info!("Switching to LeagueDetailsPanel");
            html!(
                <Suspense fallback="Loading...">
                    <LeagueDetailsPanel {league_id} />
                </Suspense>
            )
        }
        LeagueRoute::Matches(_) => {
            info!("Switching to MatchesPanel");
            html!(
                <Suspense fallback="Loading...">
                    <MatchesPanel {league_id} />
                </Suspense>
            )
        }
        LeagueRoute::Match {
            match_id,
            ..
        } => {
            info!("Switching to MatchPanel");
            html!(
                <Suspense fallback="Loading...">
                    <MatchDetailsPanel {league_id} {match_id} />
                </Suspense>
            )
        }
    };

    html!({ route })
}
