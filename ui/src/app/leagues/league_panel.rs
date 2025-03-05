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
    AppRoute,
    PageContent,
};

use super::{
    LeagueRoute,
    MatchesManagementRoute,
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
            <Scope<AppRoute,LeagueRoute> mapper={move |_| { AppRoute::mapper_league(league_id) }}>
                <RouterSwitch<LeagueRoute>
                    render={move |target| { switch_league_panel(league_id, target)}}
                />
            </Scope<AppRoute,LeagueRoute>>
        </>
    }
}

pub(crate) fn switch_league_panel(league_id: Uuid, target: LeagueRoute) -> Html {
    let route = match target {
        LeagueRoute::Details => {
            html!(
                <PageContent title={format!("League {league_id}")}>
                    <Content>
                        { format!("League {league_id} details.") }
                    </Content>
                    <Content>
                        <Link<LeagueRoute>
                            to={LeagueRoute::Matches { action: MatchesManagementRoute::Index}}
                        >
                            { format!("Link to {league_id} match list") }
                        </Link<LeagueRoute>>
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Create => {
            html!(
                <PageContent title="Create League">
                    <Content>
                        { format!("Create league.") }
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Matches {
            action: MatchesManagementRoute::Index,
        } => {
            let match_id = Uuid::new_v4();
            html!(
                <PageContent title={format!("League {league_id}")} subtitle="Matches">
                    <Content>
                        { format!("Matches for league: ") }
                        <Link<AppRoute>
                            to={AppRoute::League { league_id, details: LeagueRoute::Details}}
                        >
                            { league_id.to_string() }
                            { "." }
                        </Link<AppRoute>>
                    </Content>
                    <Content>
                        <Link<LeagueRoute> to={LeagueRoute::Match { match_id  }}>
                            { format!("Link to match {match_id}.") }
                        </Link<LeagueRoute>>
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Matches {
            action: MatchesManagementRoute::Create,
        } => {
            html!(
                <PageContent title="Create Match">
                    <Content>
                        { format!("Create match for league: {league_id}") }
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Match {
            match_id,
        } => {
            html!(
                <PageContent
                    title={format!("League {league_id}")}
                    subtitle={format!("Match {match_id}")}
                >
                    <Content>
                        { format!("League: {league_id}") }
                    </Content>
                    <Content>
                        { format!("Match: {match_id}") }
                    </Content>
                </PageContent>
            )
        }
    };

    html!({ route })
}
