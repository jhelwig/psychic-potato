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
use yew_nested_router::components::Link;

use crate::app::{
    AppRoute,
    PageContent,
    leagues::{
        LeagueRoute,
        LeaguesRoute,
        league_details_panel::fetch_league,
    },
    matches::{
        MatchRoute,
        fetch_match,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsPanelProps {
    pub league_id: Uuid,
    pub match_id:  Uuid,
}

#[function_component(MatchDetailsPanel)]
pub fn match_details_panel(props: &MatchDetailsPanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let match_id = props.match_id;
    let league_future = use_future(|| async move { fetch_league(league_id).await })?;
    let match_object_future = use_future(|| async move { fetch_match(league_id, match_id).await })?;

    let league = match &*league_future {
        Ok(league) => Rc::new(league.clone()),
        Err(error) => {
            return Ok(html!(
                <Content>
                    { format!("Error fetching league: {error}") }
                </Content>
            ));
        }
    };
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

    let html_content = html!(<MatchDetails {league} {match_object} />);

    Ok(html_content)
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchDetailsProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(MatchDetails)]
pub fn match_details(props: &MatchDetailsProps) -> Html {
    let league_name = &props.league.name;
    let match_name = &props.match_object.name;

    html!(
        <>
            <PageSection
                r#type={PageSectionType::Breadcrumbs}
                variant={PageSectionVariant::Default}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Breadcrumb>
                    <BreadcrumbItem>
                        <Link<AppRoute> to={AppRoute::Index}>
                            { "Home" }
                        </Link<AppRoute>>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <Link<AppRoute> to={AppRoute::Leagues(LeaguesRoute::Index)}>
                            { "Leagues" }
                        </Link<AppRoute>>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <Link<AppRoute>
                            to={AppRoute::League { league_id: props.league.id, page: LeagueRoute::Details }}
                        >
                            { league_name.clone() }
                        </Link<AppRoute>>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <Link<LeagueRoute>
                            to={LeagueRoute::Match { match_id: props.match_object.id, page: MatchRoute::Details }}
                        >
                            { "Matches" }
                        </Link<LeagueRoute>>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        { match_name.clone() }
                    </BreadcrumbItem>
                </Breadcrumb>
            </PageSection>
            <PageContent title={league_name.clone()} subtitle={match_name.clone()}>
                <Content>
                    { "Match Details..." }
                </Content>
            </PageContent>
        </>
    )
}
