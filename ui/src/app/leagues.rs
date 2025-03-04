use std::borrow::Borrow;

use anyhow::{
    Result,
    anyhow,
};
use gloo_net::http::Request;
use log::{
    debug,
    error,
};
use patternfly_yew::prelude::*;
use shared_types::{
    request::LeagueOperation,
    response::League,
};
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::{
    Target,
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
};

use crate::app::{
    AppRoute,
    PageContent,
    matches::MatchesRoute,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum LeagueRoute {
    #[default]
    #[target(index)]
    Details,
    Create,
    Matches,
    #[target(rename = "matches")]
    Match {
        match_id: Uuid,
    },
}

impl LeagueRoute {
    pub fn mapper_league(league_id: Uuid) -> Mapper<LeagueRoute, MatchesRoute> {
        let downwards = move |page| {
            match page {
                LeagueRoute::Matches => {
                    Some(MatchesRoute::Matches {
                        league_id,
                    })
                }
                _ => None,
            }
        };
        let upwards = move |_| LeagueRoute::Details;

        Mapper::new(downwards, upwards)
    }
}

pub fn leagues_nav_menu() -> Html {
    html_nested! {
        <>
            <NavRouterItem<AppRoute>
                to={AppRoute::Leagues { action: crate::app::LeaguesManagementRoute::Index}}
            >
                { "Leagues" }
            </NavRouterItem<AppRoute>>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeaguePanelProps {
    pub league_id: Uuid,
}

#[function_component(LeaguePanel)]
pub fn league_panel(props: &LeaguePanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <>
            <Scope<AppRoute,LeagueRoute> mapper={move |_| { AppRoute::mapper_league(league_id) }}>
                <RouterSwitch<LeagueRoute> render={move |t| { switch_league_panel(league_id, t)}} />
            </Scope<AppRoute,LeagueRoute>>
        </>
    }
}

#[function_component(LeaguesPanel)]
pub fn leagues_panel() -> Html {
    html! {
        <PageContent title="Leagues">
            <Content>
                <Suspense fallback={html!({"Loading..."})}>
                    <LeagueList />
                </Suspense>
            </Content>
        </PageContent>
    }
}

fn switch_league_panel(league_id: Uuid, target: LeagueRoute) -> Html {
    let route = match target {
        LeagueRoute::Details => {
            html!(
                <PageContent title={format!("League {league_id}")}>
                    <Content>
                        { format!("League {league_id} details.") }
                    </Content>
                    <Content>
                        <Link<LeagueRoute> to={LeagueRoute::Matches}>
                            { format!("Link to {league_id} match list") }
                        </Link<LeagueRoute>>
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Create => {
            html!(
                <PageContent title="Create League">
                    <Content>
                        { format!("Create league.") }
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Matches => {
            let match_id = Uuid::new_v4();
            html!(
                <PageContent title={format!("League {league_id}")} subtitle="Matches">
                    <Content>
                        { format!("Matches for league: ") }
                        <Link<AppRoute>
                            to={AppRoute::League { league_id, details: LeagueRoute::Details}}
                        >
                            { league_id.to_string() }
                            { "." }
                        </Link<AppRoute>>
                    </Content>
                    <Content>
                        <Link<LeagueRoute> to={LeagueRoute::Match { match_id  }}>
                            { format!("Link to match {match_id}.") }
                        </Link<LeagueRoute>>
                    </Content>
                </PageContent>
            )
        }
        LeagueRoute::Match {
            match_id,
        } => {
            html!(
                <PageContent
                    title={format!("League {league_id}")}
                    subtitle={format!("Match {match_id}")}
                >
                    <Content>
                        { format!("League: {league_id}") }
                    </Content>
                    <Content>
                        { format!("Match: {match_id}") }
                    </Content>
                </PageContent>
            )
        }
    };

    html!({ route })
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct LeagueDetailsPanelProps {
    pub league_id: Uuid,
}

#[function_component(LeagueDetailsPanel)]
pub fn league_details_panel(props: &LeagueDetailsPanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let league_result = use_future(|| async move { fetch_league(league_id).await })?;

    let html_result = match &*league_result {
        Ok(league) => html!(<LeagueDetails league={league.clone()} />),
        Err(e) => {
            html!(
                <Content>
                    { format!("Error: {e}") }
                </Content>
            )
        }
    };

    Ok(html_result)
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
struct LeagueDetailsProps {
    pub league: League,
}

#[function_component(LeagueDetails)]
fn league_details(props: &LeagueDetailsProps) -> Html {
    let League {
        id,
        name,
        created_at,
    } = &props.league;

    html! {
        <PageContent title="League Details">
            <Content>
                { format!("ID: {id}") }
            </Content>
            <Content>
                { format!("Name: {name}") }
            </Content>
            <Content>
                { format!("Created: {created_at}") }
            </Content>
        </PageContent>
    }
}

#[function_component(LeagueList)]
fn league_list() -> HtmlResult {
    let leagues_result = use_future(|| async { fetch_leagues().await })?;
    // let fake_league_id = Uuid::new_v4();

    let html_result = match &*leagues_result {
        Ok(l) => {
            let leagues = l.clone();
            html!(
                <>
                    // <Link<AppRoute>
                    //     to={AppRoute::League {
                    //     league_id: fake_league_id,
                    //     details: LeagueRoute::Details,
                    // }}
                    // >
                    //     { format!("Link to {fake_league_id} route") }
                    // </Link<AppRoute>>
                    <Content>
                        <Scope<AppRoute,LeagueRoute> mapper={AppRoute::mapper_leagues_create}>
                            <Link<LeagueRoute> to={LeagueRoute::Create}>
                                <Button
                                    variant={ButtonVariant::Primary}
                                    label="New League"
                                    icon={Icon::PlusCircle}
                                    align={Align::Start}
                                />
                            </Link<LeagueRoute>>
                        </Scope<AppRoute,LeagueRoute>>
                    </Content>
                    <Content>
                        <LeagueListTable {leagues} />
                    </Content>
                </>
            )
        }
        Err(e) => {
            html!(
                <>
                    { e.to_string() }
                </>
            )
        }
    };

    Ok(html_result)
}

#[function_component(Index)]
fn leagues_index() -> Html {
    // lsdkjlfdsjk
    html!({ "Leagues Index Component." })
}

async fn fetch_leagues() -> Result<Vec<League>> {
    let request = Request::get("/api/league").send().await?;
    let leagues: Vec<League> = request.json().await?;

    Ok(leagues)
}

async fn fetch_league(league_id: Uuid) -> Result<League> {
    let response = Request::get(&format!("/api/league/{league_id}")).send().await?;
    let league: League = if response.ok() {
        response.json().await?
    } else {
        return Err(anyhow!(
            "Failed to fetch league: {}\n{}",
            response.status(),
            response.text().await?
        ));
    };

    Ok(league)
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LeagueListTableProps {
    pub leagues: Vec<League>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LeagueListTableColumn {
    Id,
    Name,
    CreatedAt,
}

impl TableEntryRenderer<LeagueListTableColumn> for League {
    fn render_cell(&self, context: CellContext<'_, LeagueListTableColumn>) -> Cell {
        match context.column {
            LeagueListTableColumn::Id => html!(self.id.to_string()).into(),
            LeagueListTableColumn::Name => html!(
                <Link<AppRoute>
                    to={AppRoute::League { league_id: self.id, details: LeagueRoute::Details, }}
                >
                    { self.name.clone() }
                </Link<AppRoute>>
            ).into(),
            LeagueListTableColumn::CreatedAt => {
                html!(self.created_at.format("%Y-%m-%d %H:%M:%S")).into()
            }
        }
    }
}

#[function_component(LeagueListTable)]
pub fn league_list_table(props: &LeagueListTableProps) -> Html {
    let leagues_data = use_state_eq(|| props.leagues.clone());

    let on_sort_by = {
        let leagues_data = leagues_data.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<LeagueListTableColumn>| {
            let mut entries_sorted = (*leagues_data).clone();

            match column.index {
                LeagueListTableColumn::Id => entries_sorted.sort_by_key(|val| val.id),
                LeagueListTableColumn::Name => entries_sorted.sort_by_key(|val| val.name.clone()),
                LeagueListTableColumn::CreatedAt => {
                    entries_sorted.sort_by_key(|val| val.created_at)
                }
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            leagues_data.set(entries_sorted);
        }))
    };

    let (entries, _) = use_table_data(UseStateTableModel::new(leagues_data));

    let header = html_nested! {
        <TableHeader<LeagueListTableColumn>>
            <TableColumn<LeagueListTableColumn>
                label="Name"
                index={LeagueListTableColumn::Name}
                onsort={on_sort_by.clone()}
            />
            <TableColumn<LeagueListTableColumn>
                label="Created At"
                index={LeagueListTableColumn::CreatedAt}
                onsort={on_sort_by.clone()}
            />
        </TableHeader<LeagueListTableColumn>>
    };

    html! {
        <Table<LeagueListTableColumn,UseTableData<LeagueListTableColumn,UseStateTableModel<League>>>
            mode={TableMode::Compact}
            {header}
            {entries}
        />
    }
}

#[function_component(CreateLeaguePanel)]
pub fn create_league_panel() -> HtmlResult {
    let league_name = use_state_eq(String::new);
    let is_creating = use_state_eq(|| false);
    let maybe_league: UseStateHandle<Option<Result<League, String>>> = use_state_eq(|| None);
    let maybe_router = use_router::<AppRoute>();

    let onchange = use_callback(league_name.clone(), |new_league_name, league_name| {
        league_name.set(new_league_name);
    });

    let toaster = use_toaster();

    let onsubmit = {
        let league_name = league_name.clone();
        let is_creating = is_creating.setter();
        let maybe_league_setter = maybe_league.setter();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_creating.set(true);

            // Create league using league_name
            let league_operation = LeagueOperation::Create {
                league_name: (*league_name).clone(),
            };

            let spawned_league_name = league_name.clone();
            let spawned_maybe_league_setter = maybe_league_setter.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = match Request::post("/api/league/operation").json(&league_operation)
                {
                    Ok(req) => req.send().await,
                    Err(error) => {
                        error!("Unable to set request body: {}", error);
                        spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                        return;
                    }
                };
                match response {
                    Ok(response) => {
                        if response.ok() {
                            let league: League = match response.json().await {
                                Ok(league) => {
                                    spawned_league_name.set(String::new());
                                    league
                                }
                                Err(error) => {
                                    error!("Unable to parse response: {}", error);
                                    spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                                    return;
                                }
                            };
                            debug!("Created league: {league:?}");
                            spawned_maybe_league_setter.set(Some(Ok(league)));
                        } else {
                            error!("Failed to create league: {}", response.status());
                            let error_text = match response.text().await {
                                Ok(text) => text,
                                Err(error) => error.to_string(),
                            };
                            spawned_maybe_league_setter.set(Some(Err(format!(
                                "{} {}: {error_text}",
                                response.status(),
                                response.status_text()
                            ))));
                        }
                    }
                    Err(error) => {
                        error!("Error creating league: {}", error);
                        spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                    }
                }
            });

            is_creating.set(false);
            // Navigate to league details page
        })
    };

    use_effect_with(maybe_league.clone(), move |_| {
        if let Some(toaster) = toaster.borrow() {
            if let Some(league_result) = (*maybe_league).borrow() {
                let (alert_type, title, body) = match league_result {
                    Ok(league) => {
                        if let Some(router) = maybe_router {
                            debug!("Navigating to league details page: {league:?}");
                            router.push(AppRoute::League {
                                league_id: league.id,
                                details:   LeagueRoute::Details,
                            });
                        }

                        (
                            AlertType::Success,
                            "League Created",
                            html!(
                                { format!("League {} created successfully.", league.name.clone()) }
                            ),
                        )
                    }
                    Err(error) => {
                        (
                            AlertType::Danger,
                            "Error Creating League",
                            html!(
                                <>
                                    <p>
                                        { "An error occurred while creating the league." }
                                    </p>
                                    <p>
                                        { error }
                                    </p>
                                </>
                            ),
                        )
                    }
                };

                toaster.toast(Toast {
                    title: title.to_string(),
                    r#type: alert_type,
                    timeout: None,
                    body,
                    actions: Vec::new(),
                });
            }
        }
    });

    let html_content = html!(
        <PageContent title="Create League">
            <Content>
                <Form {onsubmit}>
                    <FormGroup label="Legue Name" required=true>
                        <TextInput
                            placeholder="Enter league name"
                            required=true
                            autofocus=true
                            value={(*league_name).clone()}
                            {onchange}
                        />
                    </FormGroup>
                    <ActionGroup>
                        <Button
                            variant={ButtonVariant::Primary}
                            label="Submit"
                            r#type={ButtonType::Submit}
                            icon={Icon::PlusCircle}
                            loading={*is_creating}
                        />
                    </ActionGroup>
                </Form>
            </Content>
        </PageContent>
    );

    Ok(html_content)
}
