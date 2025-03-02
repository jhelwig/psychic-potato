use anyhow::Result;
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::Match;
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

use crate::app::leagues::LeagueRoute;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesRoute {
    #[default]
    #[target(index)]
    Index,
    Matches {
        league_id: Uuid,
    },
    Match {
        match_id: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchesPanelProps {
    pub league_id: Uuid,
}

#[function_component(MatchesPanel)]
pub fn matches_panel(props: &MatchesPanelProps) -> Html {
    let league_id = props.league_id;
    html! {
        <Scope<LeagueRoute,MatchesRoute>
            mapper={move |_| { LeagueRoute::mapper_league(league_id) }}
        >
            <RouterSwitch<MatchesRoute> render={switch_matches_panel} />
        </Scope<LeagueRoute,MatchesRoute>>
    }
}

fn switch_matches_panel(target: MatchesRoute) -> Html {
    let route = match target {
        MatchesRoute::Index => {
            html!(
                <>
                    <Content>
                        { "Matches" }
                    </Content>
                    <Suspense fallback={html!({"Loading..."})}>
                        <Index />
                    </Suspense>
                </>
            )
        }
        MatchesRoute::Matches {
            league_id,
        } => {
            html!(
                <>
                    <Suspense
                        fallback={html!({format!("Loading matches for league: {league_id}")})}
                    >
                        <MatchesForLeague {league_id} />
                    </Suspense>
                </>
            )
        }
        MatchesRoute::Match {
            match_id,
        } => {
            html!({ format!("Match page: Match {match_id}") })
        }
    };

    html!({ route })
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct MatchesForLeagueProps {
    pub league_id: Uuid,
}

#[function_component(MatchesForLeague)]
fn matches_for_league_index(props: &MatchesForLeagueProps) -> HtmlResult {
    let league_id = props.league_id;
    let matches_result = use_future(|| async move { fetch_matches(league_id).await })?;
    let html_result = match &*matches_result {
        Ok(m) => {
            let matches = m.clone();
            html!(
                <>
                    <MatchListTable {matches} />
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
fn matches_index() -> Html {
    //lsdkfjsdkfj
    html!({ "Matches Index Component." })
}

async fn fetch_matches(league_id: Uuid) -> Result<Vec<Match>> {
    let request =
        Request::get("/api/match").query([("league_id", &league_id.to_string())]).send().await?;
    let matches: Vec<Match> = request.json().await?;

    Ok(matches)
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchListTableProps {
    pub matches: Vec<Match>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MatchListTableColumn {
    Id,
    Name,
    EventDate,
}

impl TableEntryRenderer<MatchListTableColumn> for Match {
    fn render_cell(&self, context: CellContext<'_, MatchListTableColumn>) -> Cell {
        match context.column {
            MatchListTableColumn::Id => html!(self.id.to_string()).into(),
            MatchListTableColumn::Name => {
                let match_id = self.id;
                html!(
                    <Link<MatchesRoute> to={MatchesRoute::Match { match_id }}>
                        { &self.name }
                    </Link<MatchesRoute>>
                )
                .into()
            }
            MatchListTableColumn::EventDate => html!(self.event_date.format("%Y-%m-%d")).into(),
        }
    }
}

#[function_component(MatchListTable)]
pub fn match_list_table(props: &MatchListTableProps) -> Html {
    let matches_data = use_state_eq(|| props.matches.clone());

    let on_sort_by = {
        let matches_data = matches_data.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<MatchListTableColumn>| {
            let mut entries_sorted = (*matches_data).clone();

            match column.index {
                MatchListTableColumn::Name => entries_sorted.sort_by_key(|val| val.name.clone()),
                MatchListTableColumn::EventDate => entries_sorted.sort_by_key(|val| val.event_date),
                _ => {}
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            matches_data.set(entries_sorted);
        }))
    };

    let (entries, _) = use_table_data(UseStateTableModel::new(matches_data));

    let header = html_nested! {
        <TableHeader<MatchListTableColumn>>
            <TableColumn<MatchListTableColumn>
                label="Name"
                index={MatchListTableColumn::Name}
                onsort={on_sort_by.clone()}
            />
            <TableColumn<MatchListTableColumn>
                label="Event Date"
                index={MatchListTableColumn::EventDate}
                onsort={on_sort_by.clone()}
            />
        </TableHeader<MatchListTableColumn>>
    };

    html! {
        <Table<MatchListTableColumn,UseTableData<MatchListTableColumn,UseStateTableModel<Match>>>
            mode={TableMode::Compact}
            {header}
            {entries}
        />
    }
}
