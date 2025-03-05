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
use yew_nested_router::{
    components::Link,
    prelude::*,
};

use crate::app::{
    leagues::LeagueRoute,
    matches::MatchesRoute,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub(crate) struct LeagueMatchListProps {
    pub league_id: Uuid,
}

#[function_component(LeagueMatchList)]
pub(crate) fn league_match_list(props: &LeagueMatchListProps) -> HtmlResult {
    let league_id = props.league_id;
    let matches_result = use_future(|| async move { fetch_matches(league_id).await })?;

    let html_result = match &*matches_result {
        Ok(matches) => {
            html!(
                <>
                    <Content>
                        <Scope<LeagueRoute,MatchesRoute>
                            mapper={move |_| { LeagueRoute::mapper_match_create(league_id)}}
                        >
                            <Link<MatchesRoute> to={MatchesRoute::Create { league_id }}>
                                <Button
                                    variant={ButtonVariant::Primary}
                                    label="Create Match"
                                    icon={Icon::PlusCircle}
                                    align={Align::Start}
                                />
                            </Link<MatchesRoute>>
                        </Scope<LeagueRoute,MatchesRoute>>
                    </Content>
                    <Content>
                        { format!("Match list table goes here. {} matches found.", matches.len()) }
                        // <MatchListTable {matches} />
                    </Content>
                </>
            )
        }
        Err(e) => {
            html!(
                <>
                    { e.to_string() }
                </>
            )
        }
    };

    Ok(html_result)
}

pub(crate) async fn fetch_matches(league_id: Uuid) -> Result<Vec<Match>> {
    let response = Request::get(&format!("/api/league/{league_id}/match")).send().await?;
    let matches: Vec<Match> = if response.ok() {
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
