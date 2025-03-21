use std::rc::Rc;

use log::error;
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
    matches::MatchRoute,
    shot_strings::{
        ShotStringRoute,
        fetch_shot_string,
        shot_string_svg_panel::ShotStringSvgPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShotStringPanelProps {
    pub league:         Rc<League>,
    pub match_object:   Rc<Match>,
    pub shot_string_id: Uuid,
}
#[function_component(ShotStringPanel)]
pub fn shot_string_panel(props: &ShotStringPanelProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let match_object = props.match_object.clone();
    let match_id = match_object.id;
    let shot_string_id = props.shot_string_id;
    let shot_string_future =
        use_future(|| async move { fetch_shot_string(league_id, match_id, shot_string_id).await })?;

    let shot_string = match &*shot_string_future {
        Ok(shot_string) => Rc::new(shot_string.clone()),
        Err(error) => {
            error!("Error fetching shot string: {}", error);
            return Ok(html!(
                <Content>
                    { "Error fetching shot string." }
                </Content>
            ));
        }
    };

    Ok(html! {
        <PageContent
            title={format!("{} ({})", shot_string.string_name.clone(), shot_string.score.to_string())}
        >
            <Scope<MatchRoute,ShotStringRoute>
                mapper={move |_| {MatchRoute::mapper_shot_string(match_id)}}
            >
                <PageSection>
                    { "Tab Section" }
                </PageSection>
                <PageSection>
                    <ShotStringSvgPanel {league} {match_object} {shot_string_id} />
                </PageSection>
            </Scope<MatchRoute,ShotStringRoute>>
        </PageContent>
    })
}
