use std::rc::Rc;

use patternfly_yew::{
    prelude::*,
    utils::Raw,
};
use shared_types::response::League;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeagueDetailsPanelProps {
    pub league: Rc<League>,
}

#[function_component(LeagueDetailsPanel)]
pub fn league_details_panel(props: &LeagueDetailsPanelProps) -> Html {
    let league = props.league.clone();

    let description = match &league.description {
        Some(description) if !description.is_empty() => {
            html_nested!(
                <>
                    <DescriptionGroup term="Description">
                        <p>
                            { &league.description }
                        </p>
                    </DescriptionGroup>
                </>
            )
        }
        _ => html_nested!(
            <></>
        ),
    };

    let dates = if league.start_date.is_some() || league.end_date.is_some() {
        html_nested!(
            <>
                <DescriptionGroup term="Dates">
                    <List r#type={ListType::Basic}>
                        <Raw>
                            if let Some(start_date) = &league.start_date {
                                <ListItem>
                                    { format!("Start: {start_date}") }
                                </ListItem>
                            }
                            if let Some(end_date) = &league.end_date {
                                <ListItem>
                                    { format!("End: {end_date}") }
                                </ListItem>
                            }
                        </Raw>
                    </List>
                </DescriptionGroup>
            </>
        )
    } else {
        html_nested!(
            <></>
        )
    };

    html! {
        <Content>
            <DescriptionList>
                { description }
                { dates }
            </DescriptionList>
        </Content>
    }
}
