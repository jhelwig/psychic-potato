use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(MatchDetailsPanel)]
pub fn match_details_panel(props: &MatchDetailsPanelProps) -> Html {
    let match_name = &props.match_object.name;

    let description = html_nested!(
        <DescriptionGroup term="Name">
            <p>
                { match_name }
            </p>
        </DescriptionGroup>
    );

    let date = html_nested!(
        <DescriptionGroup term="Date">
            <p>
                { props.match_object.event_date.to_string() }
            </p>
        </DescriptionGroup>
    );

    html!(
        <Content>
            <DescriptionList>
                { description }
                { date }
            </DescriptionList>
        </Content>
    )
}
