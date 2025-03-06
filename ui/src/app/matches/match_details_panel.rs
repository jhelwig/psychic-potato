use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
};
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::prelude::*;

use crate::app::{
    PageContent,
    leagues::league_details_panel::fetch_league,
    matches::fetch_match,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsPanelProps {
    pub league_id: Uuid,
    pub match_id:  Uuid,
}

#[function_component(MatchDetailsPanel)]
pub fn match_details_panel(props: &MatchDetailsPanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let match_id = props.match_id;
    let league_future = use_future(|| async move { fetch_league(league_id).await })?;
    let match_object_future = use_future(|| async move { fetch_match(league_id, match_id).await })?;

    let league = match &*league_future {
        Ok(league) => Rc::new(league.clone()),
        Err(error) => {
            return Ok(html!(
                <Content>
                    { format!("Error fetching league: {error}") }
                </Content>
            ));
        }
    };
    let match_object = match &*match_object_future {
        Ok(match_object) => Rc::new(match_object.clone()),
        Err(error) => {
            return Ok(html!(
                <Content>
                    { format!("Error fetching match: {error}") }
                </Content>
            ));
        }
    };

    let html_content = html!(<MatchDetails {league} {match_object} />);

    Ok(html_content)
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(MatchDetails)]
pub fn match_details(props: &MatchDetailsProps) -> Html {
    let league_name = &props.league.name;
    let match_name = &props.match_object.name;

    html!(
        <PageContent title={league_name.clone()} subtitle={match_name.clone()}>
            <Content>
                { "Match Details..." }
            </Content>
        </PageContent>
    )
}
