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
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    PageContent,
    leagues::LeagueRoute,
    matches::{
        MatchRoute,
        fetch_match,
        match_details_panel::MatchDetailsPanel,
    },
    shot_strings::{
        ShotStringsRoute,
        shot_string_list_panel::ShotStringListPanel,
        shot_string_panel::ShotStringPanel,
    },
    sm_exports::{
        SmExportsRoute,
        sm_export_panel::SmExportPanel,
        sm_exports_panel::SmExportsPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchPanelProps {
    pub league:   Rc<League>,
    pub match_id: Uuid,
}

#[function_component(MatchPanel)]
pub fn match_panel(props: &MatchPanelProps) -> Html {
    let league = props.league.clone();
    let match_id = props.match_id;

    html!(
        <>
            <Suspense fallback={html!({"Loading..."})}>
                <MatchPanelBody {league} {match_id} />
            </Suspense>
        </>
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchPanelBodyProps {
    pub league:   Rc<League>,
    pub match_id: Uuid,
}

#[function_component(MatchPanelBody)]
pub fn match_details_panel(props: &MatchPanelBodyProps) -> HtmlResult {
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
    let match_name = match_object.name.clone();

    Ok(html!(
        <>
            <PageContent title={match_name}>
                <Scope<LeagueRoute,MatchRoute>
                    mapper={move |_| { LeagueRoute::mapper_match(match_id)  }}
                >
                    <PageSection>
                        <TabsRouter<MatchRoute> r#box=true>
                            <TabRouterItem<MatchRoute> to={MatchRoute::Details} title="Details" />
                            <TabRouterItem<MatchRoute>
                                to={MatchRoute::SmExports(SmExportsRoute::Index)}
                                title="SM Exports"
                            />
                            <TabRouterItem<MatchRoute>
                                to={MatchRoute::ShotStrings(ShotStringsRoute::Index)}
                                title="Shot Strings"
                            />
                        </TabsRouter<MatchRoute>>
                    </PageSection>
                    <PageSection>
                        <RouterSwitch<MatchRoute>
                            render={move |target| { switch_match_panel(league.clone(), match_object.clone(), target)}}
                        />
                    </PageSection>
                </Scope<LeagueRoute,MatchRoute>>
            </PageContent>
        </>
    ))
}

pub fn switch_match_panel(league: Rc<League>, match_object: Rc<Match>, target: MatchRoute) -> Html {
    let route = match target {
        MatchRoute::Details => {
            html!(
                <Suspense fallback="Loading match details...">
                    <MatchDetailsPanel {league} {match_object} />
                </Suspense>
            )
        }
        MatchRoute::SmExport {
            sm_export_id,
            ..
        } => {
            html!(
                <Suspense fallback="Loading match ShotMarker export...">
                    <SmExportPanel {league} {match_object} {sm_export_id} />
                </Suspense>
            )
        }
        MatchRoute::SmExports(_) => {
            html!(
                <Suspense fallback="Loading match SnotMarker exports...">
                    <SmExportsPanel {league} {match_object} />
                </Suspense>
            )
        }
        MatchRoute::ShotString {
            shot_string_id,
            ..
        } => {
            html!(
                <Suspense fallback="Loading shot string...">
                    <ShotStringPanel {league} {match_object} {shot_string_id} />
                </Suspense>
            )
        }
        MatchRoute::ShotStrings(_) => {
            html!(
                <Suspense fallback="Loading match shot strings...">
                    <ShotStringListPanel {league} {match_object} />
                </Suspense>
            )
        }
    };

    html!({ route })
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}
