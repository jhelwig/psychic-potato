use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
};
use yew::{
    prelude::*,
    suspense::use_future,
};
use yew_nested_router::{
    components::Link,
    prelude::*,
};

use crate::app::{
    leagues::LeagueRoute,
    matches::{
        MatchRoute,
        MatchesRoute,
        fetch_matches,
    },
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MatchesListPanelProps {
    pub league: Rc<League>,
}

#[function_component(MatchesListPanel)]
pub fn matches_list_panel(props: &MatchesListPanelProps) -> Html {
    let league = props.league.clone();
    html! {
        <>
            <Content>
                <Scope<LeagueRoute,MatchesRoute> mapper={LeagueRoute::mapper_matches}>
                    <Link<MatchesRoute> to={MatchesRoute::Create}>
                        <Button
                            variant={ButtonVariant::Primary}
                            label="Create Match"
                            icon={Icon::PlusCircle}
                            align={Align::Start}
                        />
                    </Link<MatchesRoute>>
                </Scope<LeagueRoute,MatchesRoute>>
            </Content>
            <Content>
                <Suspense fallback={html!({"Loading match list..."})}>
                    <MatchList {league} />
                </Suspense>
            </Content>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchListProps {
    pub league: Rc<League>,
}

#[function_component(MatchList)]
pub fn match_list(props: &MatchListProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let matches_result = use_future(|| async move { fetch_matches(league_id).await })?;

    let html_result = match &*matches_result {
        Ok(matches) => {
            let matches = Rc::new(matches.clone());
            html!(
                <Scope<LeagueRoute,MatchRoute>
                    mapper={move |_| LeagueRoute::mapper_match(league_id)}
                >
                    <MatchListTable {matches} />
                </Scope<LeagueRoute,MatchRoute>>
            )
        }
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
                html!(
                    <Link<LeagueRoute>
                        to={LeagueRoute::Match { match_id: self.id, page: MatchRoute::Details}}
                    >
                        { self.name.clone() }
                    </Link<LeagueRoute>>
                )
                .into()
            }
            MatchListTableColumn::EventDate => html!(self.event_date.to_string()).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchListTableProps {
    pub matches: Rc<Vec<Match>>,
}

#[function_component(MatchListTable)]
pub fn match_list_table(props: &MatchListTableProps) -> Html {
    let matches_data = use_state_eq(|| props.matches.as_ref().clone());

    let on_sort_by = {
        let matches_data = matches_data.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<MatchListTableColumn>| {
            let mut entries_sorted = (*matches_data).clone();

            match column.index {
                MatchListTableColumn::Id => entries_sorted.sort_by_key(|val| val.id),
                MatchListTableColumn::Name => entries_sorted.sort_by_key(|val| val.name.clone()),
                MatchListTableColumn::EventDate => entries_sorted.sort_by_key(|val| val.event_date),
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            matches_data.set(entries_sorted);
        }))
    };

    let (entries, _) = use_table_data(UseStateTableModel::new(matches_data));

    let header = html_nested!(
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
    );

    html!(
        <Table<MatchListTableColumn,UseTableData<MatchListTableColumn,UseStateTableModel<Match>>>
            mode={TableMode::Compact}
            {header}
            {entries}
        />
    )
}
