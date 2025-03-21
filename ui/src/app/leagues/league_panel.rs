use std::rc::Rc;

use log::info;
use patternfly_yew::prelude::*;
use shared_types::response::League;
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
    AppRoute,
    PageContent,
    classes::{
        ClassesRoute,
        class_details_panel::ClassDetailsPanel,
        classes_panel::ClassesPanel,
    },
    leagues::{
        LeagueRoute,
        fetch_league,
        league_details_panel::LeagueDetailsPanel,
    },
    matches::{
        MatchesRoute,
        match_panel::MatchPanel,
        matches_panel::MatchesPanel,
    },
    shooters::{
        ShootersRoute,
        shooters_panel::ShootersPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeaguePanelProps {
    pub league_id: Uuid,
}

#[function_component(LeaguePanel)]
pub fn league_panel(props: &LeaguePanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <>
            <Suspense fallback={html!({"Loading..."})}>
                <LeaguePanelBody {league_id} />
            </Suspense>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
struct LeaguePanelBodyProps {
    pub league_id: Uuid,
}

#[function_component(LeaguePanelBody)]
fn league_panel_body(props: &LeaguePanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let league_result = use_future(|| async move { fetch_league(league_id).await })?;
    let league = match &*league_result {
        Ok(league) => Rc::new(league.clone()),
        Err(error) => {
            info!("Error fetching league: {}", error);
            return Ok(html!(
                <Content>
                    { "Error fetching league." }
                </Content>
            ));
        }
    };
    let league_name = league.name.clone();

    Ok(html!(
        //
        <>
            <PageContent title={league_name}>
                <Scope<AppRoute,LeagueRoute>
                    mapper={move |_| { AppRoute::mapper_league(league_id)}}
                >
                    <PageSection>
                        <TabsRouter<LeagueRoute> r#box=true>
                            <TabRouterItem<LeagueRoute> to={LeagueRoute::Details} title="Details" />
                            <TabRouterItem<LeagueRoute>
                                to={LeagueRoute::Classes(ClassesRoute::Index)}
                                title="Classes"
                            />
                            <TabRouterItem<LeagueRoute>
                                to={LeagueRoute::Matches(MatchesRoute::Index)}
                                title="Matches"
                            />
                            <TabRouterItem<LeagueRoute>
                                to={LeagueRoute::Shooters(ShootersRoute::Index)}
                                title="Shooters"
                            />
                        </TabsRouter<LeagueRoute>>
                    </PageSection>
                    <PageSection>
                        <RouterSwitch<LeagueRoute>
                            render={move |target| { switch_league_panel(league.clone(), target)}}
                        />
                    </PageSection>
                </Scope<AppRoute,LeagueRoute>>
            </PageContent>
        </>
    ))
}

pub fn switch_league_panel(league: Rc<League>, target: LeagueRoute) -> Html {
    let route = match target {
        LeagueRoute::Class {
            class_id,
            ..
        } => {
            html!(
                <Suspense fallback="Loading class...">
                    <ClassDetailsPanel {league} {class_id} />
                </Suspense>
            )
        }
        LeagueRoute::Classes(_) => {
            html!(
                <Suspense fallback="Loading class list...">
                    <ClassesPanel {league} />
                </Suspense>
            )
        }
        LeagueRoute::Details => {
            html!(
                <Suspense fallback="Loading league details...">
                    <LeagueDetailsPanel {league} />
                </Suspense>
            )
        }
        LeagueRoute::Matches(_) => {
            html!(
                <Suspense fallback="Loading match list...">
                    <MatchesPanel {league} />
                </Suspense>
            )
        }
        LeagueRoute::Match {
            match_id,
            ..
        } => {
            html!(
                <Suspense fallback="Loading match...">
                    <MatchPanel {league} {match_id} />
                </Suspense>
            )
        }
        LeagueRoute::Shooters(_) => {
            html!(
                <Suspense fallback="Loading shooter list...">
                    <ShootersPanel {league} />
                </Suspense>
            )
        }
    };

    html!({ route })
}
