use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::League;
use yew::prelude::*;
use yew_nested_router::{
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
};

use crate::app::{
    classes::{
        ClassesRoute,
        class_create_panel::ClassCreatePanel,
        class_list_panel::ClassListPanel,
    },
    leagues::LeagueRoute,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassesPanelProps {
    pub league: Rc<League>,
}

#[function_component(ClassesPanel)]
pub fn classes_panel(props: &ClassesPanelProps) -> Html {
    let league = props.league.clone();

    html!(
        <>
            <Scope<LeagueRoute,ClassesRoute> mapper={LeagueRoute::mapper_classes}>
                <RouterSwitch<ClassesRoute>
                    render={move |target| { switch_classes_panel(league.clone(), target)}}
                />
            </Scope<LeagueRoute,ClassesRoute>>
        </>
    )
}

pub fn switch_classes_panel(league: Rc<League>, target: ClassesRoute) -> Html {
    let route = match target {
        ClassesRoute::Index => {
            html!(
                <>
                    <Content>
                        <Link<ClassesRoute> to={ClassesRoute::Create}>
                            <Button
                                variant={ButtonVariant::Primary}
                                label="Create Class"
                                icon={Icon::PlusCircle}
                                align={Align::Start}
                            />
                        </Link<ClassesRoute>>
                    </Content>
                    <Content>
                        <ClassListPanel {league} />
                    </Content>
                </>
            )
        }
        ClassesRoute::Create => html!(<ClassCreatePanel {league} />),
    };

    html!({ route })
}
