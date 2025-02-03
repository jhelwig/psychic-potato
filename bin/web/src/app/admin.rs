use leptos::{
    logging::log,
    prelude::*,
    task::spawn_local,
};
use leptos_router::components::Outlet;
use serde::{
    Deserialize,
    Serialize,
};
use thaw::*;

#[component]
pub fn AdminHome() -> impl IntoView {
    view! {
      <div>
        <h1>Admin Home</h1>
        <Outlet/>
      </div>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
      <div>
        <h1>Dashboard</h1>
        <p>Welcome to the dashboard!</p>
      </div>
    }
}

#[component]
pub fn ShotMarkerCsvUpload() -> impl IntoView {
    let custom_request = move |file_list: FileList| {
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
                        log!("{:?}", &content);

                        log!("File length: {}", content.len());
                        files.push(ShotMarkerCsvUploadFile {
                            name: file.name(),
                            content,
                        });
                    }
                }
            }

            if !files.is_empty() {
                log!("Upload: {:?}", upload_shotmarker_csv(files).await);
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
    log!("Received {} file(s).", files.len());
    log!("{:?}", files);

    Ok(())
}
