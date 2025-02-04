use chrono::NaiveDate;
use leptos::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use thaw::*;
use uuid::Uuid;

#[component]
pub fn AssignStrings() -> impl IntoView {
    view! {
        <div>
            <h2>Assign Shot Strings</h2>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"Date"</TableHeaderCell>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Target"</TableHeaderCell>
                        <TableHeaderCell>"Distance"</TableHeaderCell>
                        <TableHeaderCell>"Score"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>
                            <TableCellLayout>"2022-01-01"</TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>"John Doe"</TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>"100m"</TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>"90"</TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>"85"</TableCellLayout>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotString {
    id:       Uuid,
    match_id: Uuid,
    date:     NaiveDate,
    name:     String,
    target:   String,
    distance: String,
    score:    String,
}
#[server]
pub async fn unassigned_strings() -> Result<Vec<ShotString>, ServerFnError> {
    use leptos::server_fn::error::NoCustomError;
    use sqlx::{
        Pool,
        Sqlite,
    };
    use uuid::Uuid;

    let mut txn = use_context::<Pool<Sqlite>>()
        .ok_or(ServerFnError::<NoCustomError>::ServerError(
            "Could not get DB connection pool.".to_string(),
        ))?
        .begin()
        .await?;

    let rows = sqlx::query!(
        r#"
            SELECT
                shot_strings.id as "id: Uuid",
                match_id AS "match_id: Uuid",
                shot_strings.date AS "date: NaiveDate",
                shot_strings.name AS "name: String",
                shot_strings.target AS "target: String",
                shot_strings.distance AS "distance: String",
                shot_strings.score AS "score: String"
            FROM shot_strings
            WHERE id NOT IN (SELECT shot_string_id FROM user_shot_strings)
            ORDER BY date ASC, name ASC
        "#,
    )
    .fetch_all(&mut *txn)
    .await?;

    let mut results = Vec::new();
    for row in rows {
        if let Some(id) = row.id
            && let Some(match_id) = row.match_id
            && let Some(date) = row.date
            && let Some(name) = row.name
            && let Some(target) = row.target
            && let Some(distance) = row.distance
            && let Some(score) = row.score
        {
            results.push(ShotString {
                id,
                match_id,
                date,
                name,
                target,
                distance,
                score,
            });
        }
    }

    Ok(results)
}
