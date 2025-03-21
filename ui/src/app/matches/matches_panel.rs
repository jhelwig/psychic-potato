use std::rc::Rc;

use shared_types::response::League;
use yew::prelude::*;
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    leagues::LeagueRoute,
    matches::{
        MatchesRoute,
        match_list_panel::MatchesListPanel,
        matches_create_panel::MatchesCreatePanel,
    },
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchesPanelProps {
    pub league: Rc<League>,
}

#[function_component(MatchesPanel)]
pub fn matches_panel(props: &MatchesPanelProps) -> Html {
    let league = props.league.clone();
    html! {
        <>
            <Scope<LeagueRoute,MatchesRoute> mapper={LeagueRoute::mapper_matches}>
                <RouterSwitch<MatchesRoute>
                    render={move |target| { switch_matches_panel(league.clone(), target)}}
                />
            </Scope<LeagueRoute,MatchesRoute>>
        </>
    }
}

pub fn switch_matches_panel(league: Rc<League>, target: MatchesRoute) -> Html {
    match target {
        MatchesRoute::Index => {
            html!(<MatchesListPanel {league} />)
        }
        MatchesRoute::Create => html!(<MatchesCreatePanel {league} />),
    }
}
