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
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
    Target,
};

mod leagues;
mod matches;

use crate::app::AppRoute;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AdminRoute {
    #[default]
    Dashboard,
    Upload,
    League {
        league_id: Uuid,
    },
    Match {
        match_id: Uuid,
    },
    New {
        #[target(nested, default)]
        item: Kind,
    },
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum Kind {
    #[default]
    League,
    Match,
}

pub fn admin_nav_menu() -> Html {
    let admin_check = true;

    if admin_check {
        html_nested! {
            <>
                <NavExpandable title="Admin">
                    <NavRouterItem<AppRoute> to={AppRoute::Admin(AdminRoute::Dashboard)}>
                        { "Dashboard" }
                    </NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Admin(AdminRoute::Upload)}>
                        { "Upload" }
                    </NavRouterItem<AppRoute>>
                </NavExpandable>
            </>
        }
    } else {
        html_nested!(
            <></>
        )
    }
}

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    html! {
        <Scope<AppRoute,AdminRoute> mapper={AppRoute::mapper_admin}>
            <RouterSwitch<AdminRoute> render={switch_admin_panel} />
        </Scope<AppRoute,AdminRoute>>
    }
}

fn switch_admin_panel(target: AdminRoute) -> Html {
    let route = match target {
        AdminRoute::Dashboard => {
            html!(<Dashboard />)
        }
        AdminRoute::Upload => {
            html!(<Upload />)
        }
        AdminRoute::League {
            league_id,
        } => html!({ format!("Requested league: {league_id}") }),
        AdminRoute::Match {
            match_id,
        } => html!({ format!("Requested match: {match_id}") }),
        AdminRoute::New {
            item,
        } => html!({ format!("Requested new item: {item:?}") }),
    };

    html! { { route } }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <>
            <Content>
                { "Dashboard" }
            </Content>
            <Link<AdminRoute> to={AdminRoute::New { item: Kind::League }}>
                { "New League" }
            </Link<AdminRoute>>
            <Link<AdminRoute> to={AdminRoute::New { item: Kind::Match}}>
                { "New Match" }
            </Link<AdminRoute>>
            <Suspense fallback={html!({"Loading..."})}>
                <MatchList />
            </Suspense>
        </>
    }
}

#[function_component(Upload)]
pub fn upload() -> Html {
    html! {
        <Content>
            { "Admin panel upload" }
        </Content>
    }
}

#[function_component(MatchList)]
pub fn match_list() -> HtmlResult {
    let matches_result = use_future(|| async { fetch_matches().await })?;
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

async fn fetch_matches() -> Result<Vec<Match>> {
    let request = Request::get("/api/match").send().await?;
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
                    <Link<AdminRoute> to={AdminRoute::Match { match_id }}>
                        { &self.name }
                    </Link<AdminRoute>>
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
