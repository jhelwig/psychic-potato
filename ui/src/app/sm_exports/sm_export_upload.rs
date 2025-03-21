use std::rc::Rc;

use gloo_net::http::Request;
use gloo_utils::format::JsValueSerdeExt;
use log::info;
use patternfly_yew::prelude::*;
use shared_types::{
    self,
    request::SmCsvExportUpload,
    response::{
        League,
        Match,
    },
};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_drop;
use yew_more_hooks::hooks::use_async_with_cloned_deps;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SmExportUploadProps {
    pub league:       Rc<League>,
    pub match_object: Rc<Match>,
}

#[function_component(SmExportUpload)]
pub fn sm_export_upload(props: &SmExportUploadProps) -> Html {
    let league_id = props.league.id;
    let match_id = props.match_object.id;

    let drop_node_ref = use_node_ref();
    let file_input_ref = use_node_ref();
    let drop_content = use_state(|| None);
    let helper_text = use_state_eq(String::new);
    let file_list_text = use_state_eq(String::new);
    let upload_progress = use_state_eq(|| 0);

    let drop = use_drop(drop_node_ref.clone());
    let onchange_choose_file = {
        let file_input_ref = file_input_ref.clone();
        let drop_content = drop_content.clone();
        let file_list_text = file_list_text.setter();
        Callback::from(move |_| {
            if let Some(element) = file_input_ref.cast::<HtmlInputElement>() {
                let files = element
                    .files()
                    .map(|files| {
                        let mut r =
                            Vec::with_capacity(files.length().try_into().unwrap_or_default());
                        for i in 0..files.length() {
                            r.extend(files.get(i));
                        }
                        r
                    })
                    .unwrap_or_default();
                info!("Files: {files:?}");
                file_list_text.set(
                    files
                        .iter()
                        .map(|file| file.name().to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                drop_content.set(Some(files));
            }
        })
    };

    let processing = {
        let drop_content_setter = drop_content.setter();
        let upload_progress = upload_progress.setter();
        use_async_with_cloned_deps(
            move |content| {
                async move {
                    let content = match &*content {
                        Some(files) => {
                            for (idx, file) in files.iter().enumerate() {
                                info!("Uploading file: {}", file.name());
                                upload_progress.set(idx + 1);

                                let content = match JsFuture::from(file.text()).await {
                                    Ok(content_jsvalue) => {
                                        content_jsvalue
                                            .into_serde::<String>()
                                            .map_err(|e| e.to_string())?
                                    }
                                    Err(error_jsvalue) => {
                                        return Err(error_jsvalue
                                            .into_serde::<String>()
                                            .map_err(|e| e.to_string())?);
                                    }
                                };
                                let request_payload = serde_json::to_string(&SmCsvExportUpload {
                                    filename: file.name().to_string(),
                                    content,
                                })
                                .map_err(|e| e.to_string())?;
                                let response = Request::post(&format!(
                                    "/api/league/{league_id}/match/{match_id}/export/upload"
                                ))
                                .header("Content-Type", "application/json")
                                .body(&request_payload)
                                .map_err(|e| e.to_string())?
                                .send()
                                .await
                                .map_err(|e| e.to_string())?;

                                if !response.ok() {
                                    return Err(format!(
                                        "Failed to upload file {}: {} {}",
                                        file.name(),
                                        response.status_text(),
                                        response.text().await.map_err(|e| e.to_string())?,
                                    ));
                                }
                            }

                            drop_content_setter.set(None);
                            Ok("File(s) uploaded successfully.".to_string())
                        }
                        None => Err("No files selected.".to_string()),
                    };

                    info!("Processing: {content:?}");

                    content
                }
            },
            drop_content.clone(),
        )
    };

    let onclick_choose_file = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(element) = file_input_ref.cast::<HtmlInputElement>() {
                element.click();
            }
        })
    };

    html!(
        <>
            <div ref={drop_node_ref}>
                <Form>
                    <FormGroup label="Upload ShotMarker CSV export.">
                        <FileUpload drag_over={*drop.over}>
                            <FileUploadSelect>
                                <InputGroup>
                                    <TextInput readonly=true value={(*file_list_text).clone()} />
                                    <input
                                        ref={file_input_ref.clone()}
                                        style="display: none;"
                                        type="file"
                                        multiple=true
                                        onchange={onchange_choose_file}
                                    />
                                    <Button
                                        variant={ButtonVariant::Control}
                                        disabled=false
                                        onclick={onclick_choose_file}
                                    >
                                        { "Choose File(s)" }
                                    </Button>
                                </InputGroup>
                            </FileUploadSelect>
                            if processing.is_processing() {
                                <FileUploadDetails>
                                    <Progress
                                        description="Processing..."
                                        range={0.0..((*drop_content).as_ref().map(|f| f.len()).unwrap_or_default() as f64)}
                                        value={*upload_progress as f64}
                                        value_text={format!("{} of {}", *upload_progress, (*drop_content).as_ref().map(|f| f.len()).unwrap_or_default())}
                                    />
                                </FileUploadDetails>
                            }
                        </FileUpload>
                        { &*helper_text }
                    </FormGroup>
                </Form>
            </div>
        </>
    )
}
