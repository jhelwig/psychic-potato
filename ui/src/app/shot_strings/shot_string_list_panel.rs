use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
    ShotMarkerShotString,
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
    matches::MatchRoute,
    shot_strings::{
        ShotStringRoute,
        fetch_shot_strings,
    },
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ShotStringListPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(ShotStringListPanel)]
pub fn shot_string_list_panel(props: &ShotStringListPanelProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let match_object = props.match_object.clone();
    let match_id = match_object.id;
    let shot_strings_result =
        use_future(|| async move { fetch_shot_strings(league_id, match_id).await })?;

    let html_result = match &*shot_strings_result {
        Ok(shot_strings) => {
            let shot_strings = Rc::new(shot_strings.clone());
            html!(
                <Scope<MatchRoute,ShotStringRoute>
                    mapper={move |_| MatchRoute::mapper_shot_string(match_id)}
                >
                    <ShotStringListTable {shot_strings} />
                </Scope<MatchRoute,ShotStringRoute>>
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

#[remain::sorted]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShotStringListTableColumn {
    Distance,
    Score,
    StringDate,
    StringName,
    Target,
}

impl TableEntryRenderer<ShotStringListTableColumn> for ShotMarkerShotString {
    fn render_cell(&self, context: CellContext<'_, ShotStringListTableColumn>) -> Cell {
        match context.column {
            ShotStringListTableColumn::StringDate => html!(self.string_date.to_string()).into(),
            ShotStringListTableColumn::StringName => {
                html!(
                    <Link<MatchRoute>
                        to={MatchRoute::ShotString {
                        shot_string_id: self.id,
                        page: ShotStringRoute::Details
                    }}
                    >
                        { self.string_name.clone() }
                    </Link<MatchRoute>>
                )
                .into()
            }
            ShotStringListTableColumn::Target => html!(self.target.clone()).into(),
            ShotStringListTableColumn::Distance => html!(self.distance.to_string()).into(),
            ShotStringListTableColumn::Score => html!(self.score.to_string()).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
struct ShotStringListTableProps {
    pub shot_strings: Rc<Vec<ShotMarkerShotString>>,
}

#[function_component(ShotStringListTable)]
fn shot_string_list_table(props: &ShotStringListTableProps) -> Html {
    let shot_strings_data = use_state_eq(|| props.shot_strings.as_ref().clone());

    let onsort = {
        let shot_strings_data = shot_strings_data.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<ShotStringListTableColumn>| {
            let mut entries_sorted = (*shot_strings_data).clone();

            match column.index {
                ShotStringListTableColumn::StringDate => {
                    entries_sorted.sort_by_key(|val| val.string_date)
                }
                ShotStringListTableColumn::StringName => {
                    entries_sorted.sort_by_key(|val| val.string_name.clone())
                }
                ShotStringListTableColumn::Target => {
                    entries_sorted.sort_by_key(|val| val.target.clone())
                }
                ShotStringListTableColumn::Distance => {
                    entries_sorted.sort_by_key(|val| val.distance.clone())
                }
                ShotStringListTableColumn::Score => {
                    entries_sorted.sort_by_key(|val| val.score.clone())
                }
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            shot_strings_data.set(entries_sorted);
        }))
    };

    let (entries, _) = use_table_data(UseStateTableModel::new(shot_strings_data));

    let header = html_nested!(
        <TableHeader<ShotStringListTableColumn>>
            <TableColumn<ShotStringListTableColumn>
                label="Name"
                width={ColumnWidth::WidthMax}
                text_modifier={TextModifier::NoWrap}
                index={ShotStringListTableColumn::StringName}
                onsort={onsort.clone()}
            />
            <TableColumn<ShotStringListTableColumn>
                label="Target"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={ShotStringListTableColumn::Target}
                onsort={onsort.clone()}
            />
            <TableColumn<ShotStringListTableColumn>
                label="Distance"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={ShotStringListTableColumn::Distance}
                onsort={onsort.clone()}
            />
            <TableColumn<ShotStringListTableColumn>
                label="Score"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={ShotStringListTableColumn::Score}
                onsort={onsort.clone()}
            />
            <TableColumn<ShotStringListTableColumn>
                label="Date"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={ShotStringListTableColumn::StringDate}
                onsort={onsort.clone()}
            />
        </TableHeader<ShotStringListTableColumn>>
    );

    html!(
        <Table<ShotStringListTableColumn,UseTableData<ShotStringListTableColumn,UseStateTableModel<ShotMarkerShotString>>>
            mode={TableMode::Compact}
            {header}
            {entries}
        />
    )
}
