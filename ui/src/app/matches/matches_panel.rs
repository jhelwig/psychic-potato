use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::Match;
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::components::Link;

use crate::app::matches::MatchesRoute;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchesPanelProps {
    pub league_id: Uuid,
}

#[function_component(MatchesListPanel)]
pub fn matches_panel(props: &MatchesPanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <>
            <Content>
                <Link<MatchesRoute> to={MatchesRoute::Create { league_id }}>
                    <Button
                        variant={ButtonVariant::Primary}
                        label="Create Match"
                        icon={Icon::PlusCircle}
                        align={Align::Start}
                    />
                </Link<MatchesRoute>>
            </Content>
            <Content>
                <Suspense fallback={html!({"Loading match list..."})}>
                    <MatchListTable {league_id} />
                </Suspense>
            </Content>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchListTableProps {
    pub league_id: Uuid,
}

#[function_component(MatchListTable)]
pub fn match_list_table(props: &MatchListTableProps) -> HtmlResult {
    let league_id = props.league_id;
    let matches_result = use_future(|| async move { fetch_matches(league_id).await })?;

    let html_result = match &*matches_result {
        Ok(matches) => {
            html!({ format!("Match list table goes here. {} matches found.", matches.len()) })
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

async fn fetch_matches(league_id: Uuid) -> Result<Vec<Match>> {
    let response = Request::get(&format!("/api/league/{league_id}/match")).send().await?;
    let matches = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch matches for league {league_id}: {}\n{}",
            response.status(),
            response.text().await?,
        ));
    };

    Ok(matches)
}
