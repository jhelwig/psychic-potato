use anyhow::Result;
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use shared_types::response::League;
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::{
    components::Link,
    prelude::*,
};

use crate::app::{
    AppRoute,
    leagues::LeagueRoute,
};

#[function_component(LeagueList)]
pub(crate) fn league_list() -> HtmlResult {
    let use_future = use_future(|| async { fetch_leagues().await })?;
    let leagues_result = use_future;
    // let fake_league_id = Uuid::new_v4();

    let html_result = match &*leagues_result {
        Ok(l) => {
            let leagues = l.clone();
            html!(
                <>
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
            LeagueListTableColumn::Name => {
                html!(
                    <Link<AppRoute>
                        to={AppRoute::League { league_id: self.id, details: LeagueRoute::Details, }}
                    >
                        { self.name.clone() }
                    </Link<AppRoute>>
                )
                .into()
            }
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
