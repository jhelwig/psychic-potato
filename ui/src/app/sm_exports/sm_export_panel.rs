use std::rc::Rc;

use log::error;
use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
    ShotMarkerExport,
};
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    PageContent,
    matches::MatchRoute,
    sm_exports::{
        SmExportRoute,
        fetch_sm_export,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SmExportPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
    pub sm_export_id: Uuid,
}

#[function_component(SmExportPanel)]
pub fn sm_export_panel(props: &SmExportPanelProps) -> HtmlResult {
    let match_object = props.match_object.clone();
    let match_id = match_object.id;
    let league = props.league.clone();
    let league_id = league.id;
    let sm_export_id = props.sm_export_id;
    let sm_export_future =
        use_future(|| async move { fetch_sm_export(league_id, match_id, sm_export_id).await })?;

    let sm_export = match &*sm_export_future {
        Ok(sm_export) => Rc::new(sm_export.clone()),
        Err(error) => {
            error!("Error fetching ShotMarker export: {}", error);
            return Ok(html!(
                <Content>
                    { "Error fetching ShotMarker export." }
                </Content>
            ));
        }
    };
    let sm_file_name = sm_export.file_name.clone();

    Ok(html!(
        <>
            <PageContent title={sm_file_name}>
                <Scope<MatchRoute,SmExportRoute>
                    mapper={move |_| { MatchRoute::mapper_sm_export(match_id)  }}
                >
                    <PageSection>
                        <TabsRouter<SmExportRoute> r#box=true>
                            <TabRouterItem<SmExportRoute>
                                to={SmExportRoute::Details}
                                title="Details"
                            />
                        </TabsRouter<SmExportRoute>>
                    </PageSection>
                    <PageSection>
                        <RouterSwitch<SmExportRoute>
                            render={move |target| { switch_sm_export_panel(league.clone(), match_object.clone(), sm_export.clone(), target)}}
                        />
                    </PageSection>
                </Scope<MatchRoute,SmExportRoute>>
            </PageContent>
        </>
    ))
}

fn switch_sm_export_panel(
    _league: Rc<League>,
    _match_object: Rc<Match>,
    sm_export: Rc<ShotMarkerExport>,
    target: SmExportRoute,
) -> Html {
    match target {
        SmExportRoute::Details => {
            html!(
                <Content>
                    { format!("ShotMarker export details for {}", sm_export.file_name) }
                </Content>
            )
        }
    }
}
