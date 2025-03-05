use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::{
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
};

use crate::app::{
    PageContent,
    leagues::LeagueRoute,
};

pub mod matches_panel;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesRoute {
    #[default]
    #[target(index)]
    Details,
    Create {
        league_id: Uuid,
    },
    Matches {
        league_id: Uuid,
    },
    Match {
        match_id: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchPanelProps {
    pub league_id: Uuid,
    pub match_id:  Uuid,
}

#[function_component(MatchPanel)]
pub fn matches_panel(props: &MatchPanelProps) -> Html {
    let league_id = props.league_id;
    let match_id = props.match_id;
    html! {
        <>
            <Scope<LeagueRoute,MatchesRoute>
                mapper={move |_| LeagueRoute::mapper_matches(league_id)}
            >
                <RouterSwitch<MatchesRoute>
                    render={move |target| { switch_match_panel(league_id, match_id, target)}}
                />
            </Scope<LeagueRoute,MatchesRoute>>
        </>
    }
}

pub fn switch_match_panel(league_id: Uuid, match_id: Uuid, target: MatchesRoute) -> Html {
    let route = match target {
        MatchesRoute::Details => {
            html!(
                <PageContent title={format!("Match {match_id}")}>
                    <Content>
                        { format!("Match {match_id} details.") }
                    </Content>
                    <Content>
                        <Link<MatchesRoute> to={MatchesRoute::Matches { league_id }}>
                            { format!("Link to league {league_id}") }
                        </Link<MatchesRoute>>
                    </Content>
                </PageContent>
            )
        }
        MatchesRoute::Create {
            league_id,
        } => todo!(),
        MatchesRoute::Matches {
            league_id,
        } => todo!(),
        MatchesRoute::Match {
            match_id,
        } => todo!(),
    };

    html!({ route })
}
