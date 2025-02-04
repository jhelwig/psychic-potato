use leptos::{
    logging::log,
    prelude::*,
    task::spawn_local,
};
use serde::{
    Deserialize,
    Serialize,
};
use thaw::*;

#[component]
pub fn ShotMarkerCsvUpload() -> impl IntoView {
    // let toaster = ToasterInjection::expect_context();

    let custom_request = move |file_list: FileList| {
        // toaster.dispatch_toast(
        //     move || {
        //         view! {
        //             <Toast>
        //                 <ToastTitle>
        //                     "File(s) uploaded successfully." <ToastTitleMedia slot>
        //                         <Spinner size=SpinnerSize::Tiny />
        //                     </ToastTitleMedia>
        //                 </ToastTitle>
        //             </Toast>
        //         }
        //     },
        //     Default::default(),
        // );
        spawn_local(async move {
            let mut files = Vec::new();
            log!("Received {} file(s)", file_list.length());
            log!("Upload: {:?}", file_list);
            let mut i = 0;
            while let Some(file) = file_list.item(i) {
                i += 1;
                log!("Processing file {}: {} {} ({})", i, file.name(), file.type_(), file.size());
                if let Ok(content) = wasm_bindgen_futures::JsFuture::from(file.text()).await {
                    if let Some(content) = content.as_string() {
                        // log!("{:?}", &content);

                        log!("File length: {}", content.len());
                        files.push(ShotMarkerCsvUploadFile {
                            name: file.name(),
                            content,
                        });
                    }
                }
            }

            if !files.is_empty() {
                match upload_shotmarker_csv(files).await {
                    Ok(_) => {
                        log!("Files uploaded successfully.");
                        // toaster.dispatch_toast(
                        //     move || {
                        //         view! {
                        //             <Toast>
                        //                 <ToastBody>"File(s) uploaded successfully."</ToastBody>
                        //             </Toast>
                        //         }
                        //     },
                        //     Default::default(),
                        // );
                    }
                    Err(err) => log!("Error uploading files: {:?}", err),
                }
            }
        })
    };

    view! {
        <div>
            <h1>Upload</h1>
            <p>Upload files here.</p>
            <Upload multiple=true name="shotmarker_csv" custom_request>
                <UploadDragger>"Click or drag ShotMarker CSV export(s) here."</UploadDragger>
            </Upload>
        </div>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerCsvUploadFile {
    name:    String,
    content: String,
}

#[server]
pub async fn upload_shotmarker_csv(
    files: Vec<ShotMarkerCsvUploadFile>,
) -> Result<(), ServerFnError> {
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

    log!("Received {} file(s).", files.len());
    for file in files {
        log!("Processing file: {} ({})", file.name, file.content.len());
        let (_, shotmarker_export) = shotmarker_csv_parser::parser::export_parser(&file.content)?;
        log!("Parsed ShotMarker export successfully.");

        let export_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO matches (id, name, date) VALUES ($1, $2, $3)",
            export_id,
            shotmarker_export.string_date,
            shotmarker_export.string_date,
        )
        .execute(&mut *txn)
        .await?;

        for shot_string in shotmarker_export.strings {
            let shot_string_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO shot_strings (id, match_id, date, name, target, distance, score) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                shot_string_id,
                export_id,
                shot_string.date,
                shot_string.name,
                shot_string.target,
                shot_string.distance,
                shot_string.score,
            )
            .execute(&mut *txn)
            .await?;

            for shot in shot_string.shots {
                let shot_id = Uuid::new_v4();
                let score = serde_json::to_value(shot.score)?;
                let position = serde_json::to_value(shot.position)?;
                let velocity = serde_json::to_value(shot.velocity)?;
                let yaw = serde_json::to_value(shot.yaw)?;
                let pitch = serde_json::to_value(shot.pitch)?;
                sqlx::query!(
                    "INSERT INTO shots (id, shot_string_id, shot_string, shot_id, tags, score, position, velocity, yaw, pitch, quality) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                    shot_id,
                    shot_string_id,
                    shot.time,
                    shot.id,
                    shot.tags,
                    score,
                    position,
                    velocity,
                    yaw,
                    pitch,
                    shot.quality,
                )
               .execute(&mut *txn)
               .await?;
            }
        }
    }
    txn.commit().await?;

    Ok(())
}
