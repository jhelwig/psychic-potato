use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
    ShotMarkerShotString,
};
use yew::prelude::*;

use crate::app::shooters::shooter_autocomplete::ShooterAutocomplete;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShotStringAssignmentPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
    pub shot_string:  Rc<ShotMarkerShotString>,
}

#[function_component(ShotStringAssignmentPanel)]
pub fn shot_string_assignment_panel(props: &ShotStringAssignmentPanelProps) -> HtmlResult {
    let league = props.league.clone();
    let match_object = props.match_object.clone();
    let shot_string = props.shot_string.clone();

    Ok(html!(
        <PageSection>
            <ShooterAutocomplete
                initial_search={shot_string.string_name.clone()}
                {league}
                {match_object}
                {shot_string}
            />
        </PageSection>
    ))
}
