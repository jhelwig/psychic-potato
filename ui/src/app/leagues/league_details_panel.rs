use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::League;
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::prelude::*;

use crate::app::{
    PageContent,
    leagues::LeagueRoute,
    matches::{
        MatchesRoute,
        matches_panel::MatchesListPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeagueDetailsPanelProps {
    pub league_id: Uuid,
}

#[function_component(LeagueDetailsPanel)]
pub fn league_details_panel(props: &LeagueDetailsPanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let league_result = use_future(|| async move { fetch_league(league_id).await })?;

    let html_result = match &*league_result {
        Ok(league) => html!(<LeagueDetails league={league.clone()} />),
        Err(e) => {
            html!(
                <Content>
                    { format!("Error: {e}") }
                </Content>
            )
        }
    };

    Ok(html_result)
}

async fn fetch_league(league_id: Uuid) -> Result<League> {
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

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub(crate) struct LeagueDetailsProps {
    pub league: League,
}

#[function_component(LeagueDetails)]
pub(crate) fn league_details(props: &LeagueDetailsProps) -> Html {
    let League {
        id,
        name,
        created_at,
    } = &props.league;

    let league_id = *id;
    html! {
        <PageContent title={name.clone()} subtitle={format!("Created: {created_at}")}>
            <Content>
                <Suspense fallback={html!({"Loading match list..."})}>
                    <Scope<LeagueRoute,MatchesRoute>
                        mapper={move |_| { LeagueRoute::mapper_matches(league_id) }}
                    >
                        <MatchesListPanel {league_id} />
                    </Scope<LeagueRoute,MatchesRoute>>
                </Suspense>
            </Content>
        </PageContent>
    }
}
