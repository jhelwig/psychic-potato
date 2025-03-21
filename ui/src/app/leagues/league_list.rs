use anyhow::Result;
use gloo_net::http::Request;
use log::info;
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
    leagues::{
        LeagueRoute,
        LeaguesRoute,
    },
};

#[function_component(LeagueList)]
pub fn league_list() -> HtmlResult {
    let use_future = use_future(|| async { fetch_leagues().await })?;
    let leagues_result = use_future;
    // let fake_league_id = Uuid::new_v4();

    let html_result = match &*leagues_result {
        Ok(l) => {
            let leagues = l.clone();
            html!(
                <>
                    <Content>
                        <Scope<AppRoute,LeaguesRoute> mapper={AppRoute::mapper_leagues}>
                            <Link<LeaguesRoute> to={LeaguesRoute::Create}>
                                <Button
                                    variant={ButtonVariant::Primary}
                                    label="New League"
                                    icon={Icon::PlusCircle}
                                    align={Align::Start}
                                />
                            </Link<LeaguesRoute>>
                        </Scope<AppRoute,LeaguesRoute>>
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

#[remain::sorted]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LeagueListTableColumn {
    Actions,
    CreatedAt,
    Id,
    Name,
}

impl TableEntryRenderer<LeagueListTableColumn> for League {
    fn render_cell(&self, context: CellContext<'_, LeagueListTableColumn>) -> Cell {
        match context.column {
            LeagueListTableColumn::Id => html!(self.id.to_string()).into(),
            LeagueListTableColumn::Name => {
                html!(
                    <Link<AppRoute>
                        to={AppRoute::League { league_id: self.id, page: LeagueRoute::Details }}
                    >
                        { self.name.clone() }
                    </Link<AppRoute>>
                )
                .into()
            }
            LeagueListTableColumn::CreatedAt => {
                Cell::from(html!(self.created_at.format("%Y-%m-%d %H:%M:%S")))
                    .text_modifier(TextModifier::NoWrap)
            }
            LeagueListTableColumn::Actions => {
                let onclick = {
                    let league_id = self.id;

                    Callback::from(move |_| {
                        info!("Clicked delete! League ID: {}", league_id);
                    })
                };
                Cell::from(html!(
                    <Button variant={ButtonVariant::DangerSecondary} {onclick}>
                        { "Delete" }
                    </Button>
                ))
                .text_modifier(TextModifier::NoWrap)
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
                // LeagueListTableColumn::Id => entries_sorted.sort_by_key(|val| val.id),
                LeagueListTableColumn::Name => entries_sorted.sort_by_key(|val| val.name.clone()),
                LeagueListTableColumn::CreatedAt => {
                    entries_sorted.sort_by_key(|val| val.created_at)
                }
                _ => {}
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
                width={ColumnWidth::WidthMax}
                index={LeagueListTableColumn::Name}
                onsort={on_sort_by.clone()}
            />
            <TableColumn<LeagueListTableColumn>
                label="Created At"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={LeagueListTableColumn::CreatedAt}
                onsort={on_sort_by.clone()}
            />
            <TableColumn<LeagueListTableColumn>
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={LeagueListTableColumn::Actions}
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
