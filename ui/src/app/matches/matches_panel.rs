use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    leagues::{
        LeagueRoute,
        league_details_panel::LeagueDetailsPanel,
    },
    matches::MatchesRoute,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchesPanelProps {
    pub league_id: Uuid,
}

#[function_component(MatchesPanel)]
pub fn matches_panel(props: &MatchesPanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <>
            <Scope<LeagueRoute,MatchesRoute> mapper={LeagueRoute::mapper_matches}>
                <RouterSwitch<MatchesRoute>
                    render={move |target| { switch_matches_panel(league_id, target)}}
                />
            </Scope<LeagueRoute,MatchesRoute>>
        </>
    }
}

pub fn switch_matches_panel(league_id: Uuid, target: MatchesRoute) -> Html {
    let route = match target {
        MatchesRoute::Index => {
            html!(<LeagueDetailsPanel {league_id} />)
        }
        MatchesRoute::Create => html!({ format!("Create match for league: {league_id}") }),
    };

    html!({ route })
}
