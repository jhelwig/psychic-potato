use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
};
use yew::prelude::*;
use yew_nested_router::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    matches::MatchRoute,
    sm_exports::{
        SmExportsRoute,
        sm_export_list_panel::SmExportsListPanel,
        sm_export_upload::SmExportUpload,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SmExportsPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(SmExportsPanel)]
pub fn sm_exports_panel(props: &SmExportsPanelProps) -> Html {
    let league = props.league.clone();
    let match_object = props.match_object.clone();

    html! {
        <>
            <Scope<MatchRoute,SmExportsRoute> mapper={MatchRoute::mapper_sm_exports}>
                <RouterSwitch<SmExportsRoute>
                    render={move |target| {switch_sm_exports_panel(league.clone(), match_object.clone(), target)}}
                />
            </Scope<MatchRoute,SmExportsRoute>>
        </>
    }
}

pub fn switch_sm_exports_panel(
    league: Rc<League>,
    match_object: Rc<Match>,
    target: SmExportsRoute,
) -> Html {
    match target {
        SmExportsRoute::Index => {
            html!(<SmExportsListPanel {league} {match_object} />)
        }
        SmExportsRoute::Upload => {
            html!(<SmExportUpload {league} {match_object} />)
        }
    }
}
