use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
    ShotMarkerExport,
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
    sm_exports::{
        SmExportRoute,
        SmExportsRoute,
        fetch_sm_exports,
    },
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SmExportsListPanelProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(SmExportsListPanel)]
pub fn sm_exports_list_panel(props: &SmExportsListPanelProps) -> Html {
    let league = props.league.clone();
    let match_object = props.match_object.clone();

    html!(
        <>
            <Content>
                <Scope<MatchRoute,SmExportsRoute> mapper={MatchRoute::mapper_sm_exports}>
                    <Link<SmExportsRoute> to={SmExportsRoute::Upload}>
                        <Button
                            variant={ButtonVariant::Primary}
                            label="Upload SM Export"
                            icon={Icon::PlusCircle}
                            align={Align::Start}
                        />
                    </Link<SmExportsRoute>>
                </Scope<MatchRoute,SmExportsRoute>>
            </Content>
            <Content>
                <Suspense fallback={html!({"Loading SM export list..."})}>
                    <SmExportList {league} {match_object} />
                </Suspense>
            </Content>
        </>
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SmExportListProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(SmExportList)]
pub fn sm_export_list(props: &SmExportListProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let match_object = props.match_object.clone();
    let match_id = match_object.id;
    let sm_exports_result =
        use_future(|| async move { fetch_sm_exports(league_id, match_id).await })?;

    let html_result = match &*sm_exports_result {
        Ok(sm_exports) => {
            let sm_exports = Rc::new(sm_exports.clone());
            html!(
                <Scope<MatchRoute,SmExportRoute>
                    mapper={move |_| MatchRoute::mapper_sm_export(match_id)}
                >
                    <SmExportListTable {sm_exports} />
                </Scope<MatchRoute,SmExportRoute>>
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
pub enum SmExportListTableColumn {
    Id,
    Filename,
    GeneratedDate,
    StringCount,
}

impl TableEntryRenderer<SmExportListTableColumn> for ShotMarkerExport {
    fn render_cell(&self, context: CellContext<'_, SmExportListTableColumn>) -> Cell {
        match context.column {
            SmExportListTableColumn::Id => html!(self.id.to_string()).into(),
            SmExportListTableColumn::Filename => {
                html!(
                    <Link<MatchRoute>
                        to={MatchRoute::SmExport {
                        sm_export_id: self.id,
                        page: SmExportRoute::Details
                    }}
                    >
                        { self.file_name.clone() }
                    </Link<MatchRoute>>
                )
                .into()
            }
            SmExportListTableColumn::GeneratedDate => html!(self.generated_date.to_string()).into(),
            SmExportListTableColumn::StringCount => html!(self.string_count.to_string()).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SmExportListTableProps {
    pub sm_exports: Rc<Vec<ShotMarkerExport>>,
}

#[function_component(SmExportListTable)]
pub fn sm_export_list_table(props: &SmExportListTableProps) -> Html {
    let sm_exports_data = use_state_eq(|| props.sm_exports.as_ref().clone());

    let onsort = {
        let sm_exports_data = sm_exports_data.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<SmExportListTableColumn>| {
            let mut entries_sorted = (*sm_exports_data).clone();

            match column.index {
                SmExportListTableColumn::Id => entries_sorted.sort_by_key(|val| val.id),
                SmExportListTableColumn::Filename => {
                    entries_sorted.sort_by_key(|val| val.file_name.clone())
                }
                SmExportListTableColumn::GeneratedDate => {
                    entries_sorted.sort_by_key(|val| val.generated_date)
                }
                SmExportListTableColumn::StringCount => {
                    entries_sorted.sort_by_key(|val| val.string_count)
                }
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            sm_exports_data.set(entries_sorted);
        }))
    };

    let (entries, _) = use_table_data(UseStateTableModel::new(sm_exports_data));

    let header = html_nested!(
        <TableHeader<SmExportListTableColumn>>
            <TableColumn<SmExportListTableColumn>
                label="Filename"
                width={ColumnWidth::WidthMax}
                text_modifier={TextModifier::NoWrap}
                index={SmExportListTableColumn::Filename}
                onsort={onsort.clone()}
            />
            <TableColumn<SmExportListTableColumn>
                label="Generated Date"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={SmExportListTableColumn::GeneratedDate}
                onsort={onsort.clone()}
            />
            <TableColumn<SmExportListTableColumn>
                label="String Count"
                width={ColumnWidth::FitContent}
                text_modifier={TextModifier::NoWrap}
                index={SmExportListTableColumn::StringCount}
                {onsort}
            />
        </TableHeader<SmExportListTableColumn>>
    );

    html!(
        <Table<SmExportListTableColumn,UseTableData<SmExportListTableColumn,UseStateTableModel<ShotMarkerExport>>>
            mode={TableMode::Compact}
            {header}
            {entries}
        />
    )
}
