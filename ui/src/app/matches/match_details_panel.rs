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

use crate::app::{
    PageContent,
    matches::fetch_match,
    sm_upload::SmExportUpload,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsPanelProps {
    pub league:   Rc<League>,
    pub match_id: Uuid,
}

#[function_component(MatchDetailsPanel)]
pub fn match_details_panel(props: &MatchDetailsPanelProps) -> HtmlResult {
    let league_id = props.league.id;
    let league = props.league.clone();
    let match_id = props.match_id;
    let match_object_future = use_future(|| async move { fetch_match(league_id, match_id).await })?;

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
    let match_name = &props.match_object.name;

    html!(
        <>
            <PageContent title={match_name.clone()}>
                <Content>
                    <SmExportUpload
                        league={props.league.clone()}
                        match_object={props.match_object.clone()}
                    />
                </Content>
                <Content>
                    { "Match Details..." }
                </Content>
            </PageContent>
        </>
    )
}
