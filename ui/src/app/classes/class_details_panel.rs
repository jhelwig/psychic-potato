use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    Class,
    League,
};
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};

use crate::app::{
    PageContent,
    classes::fetch_class,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassDetailsPanelProps {
    pub league:   Rc<League>,
    pub class_id: Uuid,
}

#[function_component(ClassDetailsPanel)]
pub fn class_details_panel(props: &ClassDetailsPanelProps) -> HtmlResult {
    let league_id = props.league.id;
    let league = props.league.clone();
    let class_id = props.class_id;
    let class_future = use_future(|| async move { fetch_class(league_id, class_id).await })?;

    let class = match &*class_future {
        Ok(class) => Rc::new(class.clone()),
        Err(error) => {
            return Ok(html!(
                <Content>
                    { format!("Error fetching class: {error}") }
                </Content>
            ));
        }
    };

    let html_content = html!(<ClassDetails {league} {class} />);

    Ok(html_content)
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassDetailsProps {
    pub league: Rc<League>,
    pub class:  Rc<Class>,
}

#[function_component(ClassDetails)]
pub fn class_details(props: &ClassDetailsProps) -> Html {
    let class_name = &props.class.name;
    let rendered_description = if let Some(description) = &props.class.description {
        yew::Html::from_html_unchecked(yew::AttrValue::from(markdown::to_html(description)))
    } else {
        html!()
    };

    html!(
        <>
            <PageContent title={class_name.clone()}>
                <Content>
                    { rendered_description }
                </Content>
            </PageContent>
        </>
    )
}
