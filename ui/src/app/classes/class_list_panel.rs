use std::rc::Rc;

use patternfly_yew::prelude::*;
use shared_types::response::{
    Class,
    League,
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
    classes::{
        ClassRoute,
        fetch_classes,
    },
    leagues::LeagueRoute,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassListPanelProps {
    pub league: Rc<League>,
}

#[function_component(ClassListPanel)]
pub fn class_list_panel(props: &ClassListPanelProps) -> Html {
    let league = props.league.clone();
    html!(
        <>
            <Content>
                <Suspense fallback={html!({"Loading class list..."})}>
                    <ClassList {league} />
                </Suspense>
            </Content>
        </>
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassListProps {
    pub league: Rc<League>,
}

#[function_component(ClassList)]
pub fn class_list(props: &ClassListProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let classes_result = use_future(|| async move { fetch_classes(league_id).await })?;

    let html_result = match &*classes_result {
        Ok(classes) => {
            let classes = Rc::new(classes.clone());
            html!(
                <Scope<LeagueRoute,ClassRoute>
                    mapper={move |_| LeagueRoute::mapper_class(league_id)}
                >
                    <ClassListTable {classes} />
                </Scope<LeagueRoute,ClassRoute>>
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
pub enum ClassListTableColumn {
    Id,
    Name,
    Description,
}

impl TableEntryRenderer<ClassListTableColumn> for Class {
    fn render_cell(&self, context: CellContext<'_, ClassListTableColumn>) -> Cell {
        match context.column {
            ClassListTableColumn::Id => html!({ self.id.to_string() }).into(),
            ClassListTableColumn::Name => {
                html!(
                    <Link<LeagueRoute>
                        to={LeagueRoute::Class { class_id: self.id, page: ClassRoute::Details}}
                    >
                        { self.name.clone() }
                    </Link<LeagueRoute>>
                )
                .into()
            }
            ClassListTableColumn::Description => {
                html!(<Truncate content={self.description.clone().unwrap_or_default()} />).into()
            }
        }
    }

    fn is_full_width_details(&self) -> Option<bool> { Some(true) }

    fn render_details(&self) -> Vec<Span> {
        let Some(description) = &self.description else {
            return Vec::new();
        };
        let rendered_description = markdown::to_html(description);
        let html_description =
            yew::Html::from_html_unchecked(yew::AttrValue::from(rendered_description));
        let span = Span::max(html!(
            <p>
                { html_description }
            </p>
        ));

        vec![span]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassListTableProps {
    pub classes: Rc<Vec<Class>>,
}

#[function_component(ClassListTable)]
pub fn class_list_table(props: &ClassListTableProps) -> Html {
    let classes = use_state_eq(|| props.classes.as_ref().clone());

    let on_sort_by = {
        let classes = classes.clone();

        Some(Callback::from(move |column: TableHeaderSortBy<ClassListTableColumn>| {
            let mut entries_sorted = (*classes).clone();

            match column.index {
                ClassListTableColumn::Id => entries_sorted.sort_by_key(|val| val.id),
                ClassListTableColumn::Name => entries_sorted.sort_by_key(|val| val.name.clone()),
                ClassListTableColumn::Description => {
                    entries_sorted.sort_by_key(|val| val.description.clone().unwrap_or_default())
                }
            }

            if matches!(column.order, Order::Descending) {
                entries_sorted.reverse();
            }
            classes.set(entries_sorted);
        }))
    };

    let (entries, onexpand) = use_table_data(UseStateTableModel::new(classes));

    let header = html_nested!(
        <TableHeader<ClassListTableColumn>>
            <TableColumn<ClassListTableColumn>
                label="Name"
                index={ClassListTableColumn::Name}
                onsort={on_sort_by.clone()}
            />
            <TableColumn<ClassListTableColumn>
                label="Description"
                index={ClassListTableColumn::Description}
                onsort={on_sort_by.clone()}
            />
        </TableHeader<ClassListTableColumn>>
    );

    html!(
        <Table<ClassListTableColumn,UseTableData<ClassListTableColumn,UseStateTableModel<Class>>>
            mode={TableMode::CompactExpandable}
            {header}
            {entries}
            {onexpand}
        />
    )
}
