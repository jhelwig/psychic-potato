use anyhow::Result;
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::League;
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::{
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
    Target,
};

use crate::app::{
    matches::MatchesRoute,
    AppRoute,
    PageContent,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum LeagueRoute {
    #[default]
    #[target(index)]
    Details,
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
            <NavRouterItem<AppRoute> to={AppRoute::Leagues}>
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

#[function_component(LeagueList)]
fn league_list() -> HtmlResult {
    let leagues_result = use_future(|| async { fetch_leagues().await })?;
    let fake_league_id = Uuid::new_v4();

    let html_result = match &*leagues_result {
        Ok(l) => {
            let leagues = l.clone();
            html!(
                <>
                    <Link<AppRoute>
                        to={AppRoute::League {
                        league_id: fake_league_id,
                        details: LeagueRoute::Details,
                    }}
                    >
                        { format!("Link to {fake_league_id} route") }
                    </Link<AppRoute>>
                    <LeagueListTable {leagues} />
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
            LeagueListTableColumn::Name => html!(self.name.clone()).into(),
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
                label="ID"
                index={LeagueListTableColumn::Id}
                onsort={on_sort_by.clone()}
            />
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
