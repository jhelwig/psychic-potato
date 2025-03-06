use std::rc::Rc;

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

use crate::app::{
    PageContent,
    matches::matches_list_panel::MatchesListPanel,
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
        Ok(league) => {
            let league = Rc::new(league.clone());
            html!(<LeagueDetails {league} />)
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeagueDetailsProps {
    pub league: Rc<League>,
}

#[function_component(LeagueDetails)]
pub fn league_details(props: &LeagueDetailsProps) -> Html {
    let league = props.league.clone();

    let league_name = league.name.clone();
    let league_created_at = league.created_at;
    html! {
        <PageContent title={league_name} subtitle={format!("Created: {league_created_at}")}>
            <Content>
                <Suspense fallback={html!({"Loading match list..."})}>
                    <MatchesListPanel {league} />
                </Suspense>
            </Content>
        </PageContent>
    }
}
