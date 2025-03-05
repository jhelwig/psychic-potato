use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsPanelProps {
    pub league_id: Uuid,
    pub match_id:  Uuid,
}

#[function_component(MatchDetailsPanel)]
pub fn match_details_panel(props: &MatchDetailsPanelProps) -> Html {
    html!({ format!("Match Details for League: {}, Match: {}", props.league_id, props.match_id) })
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsProps {
    pub league_id: Uuid,
    pub match_id:  Uuid,
}

// #[function_component(MatchDetails)]
// pub fn match_details()
