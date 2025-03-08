use std::{
    borrow::Borrow,
    rc::Rc,
    time::Duration,
};

use gloo_net::http::Request;
use log::{
    debug,
    error,
};
use patternfly_yew::prelude::*;
use shared_types::{
    request::ClassOperation,
    response::{
        Class,
        League,
    },
};
use yew::prelude::*;
use yew_nested_router::prelude::*;

use crate::app::{
    PageContent,
    leagues::LeagueRoute,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ClassCreatePanelProps {
    pub league: Rc<League>,
}

#[function_component(ClassCreatePanel)]
pub fn class_create_panel(props: &ClassCreatePanelProps) -> HtmlResult {
    let league_id = props.league.id;
    let class_name = use_state_eq(String::new);
    let class_description = use_state_eq(String::new);
    let is_creating = use_state_eq(|| false);
    let maybe_class: UseStateHandle<Option<Result<Class, String>>> = use_state_eq(|| None);
    let maybe_router = use_router::<LeagueRoute>();
    let toaster = use_toaster();

    let class_name_onchange = use_callback(class_name.clone(), |new_class_name, class_name| {
        class_name.set(new_class_name);
    });

    let class_description_onchange =
        use_callback(class_description.clone(), |new_class_description, class_description| {
            class_description.set(new_class_description);
        });

    let onsubmit = {
        let class_name = class_name.clone();
        let class_description = class_description.clone();
        let is_creating = is_creating.setter();
        let maybe_class_setter = maybe_class.setter();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_creating.set(true);

            let description = (*class_description).clone();
            let trimmed_description = description.trim();
            let operation_description = if trimmed_description.is_empty() {
                None
            } else {
                Some(trimmed_description.to_string())
            };
            let class_operation = ClassOperation::Create {
                name:        (*class_name).clone(),
                description: operation_description,
            };
            let spawned_class_name = class_name.clone();
            let spawned_class_description = class_description.clone();
            let spawned_maybe_class_setter = maybe_class_setter.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    match Request::post(&format!("/api/league/{league_id}/class/operation"))
                        .json(&class_operation)
                    {
                        Ok(req) => req.send().await,
                        Err(error) => {
                            error!("Unable to set request body: {}", error);
                            spawned_maybe_class_setter.set(Some(Err(error.to_string())));
                            return;
                        }
                    };
                match response {
                    Ok(response) => {
                        if response.ok() {
                            let class: Class = match response.json().await {
                                Ok(class) => {
                                    spawned_class_name.set(String::new());
                                    spawned_class_description.set(String::new());
                                    class
                                }
                                Err(error) => {
                                    error!("Unable to parse response: {}", error);
                                    spawned_maybe_class_setter.set(Some(Err(error.to_string())));
                                    return;
                                }
                            };
                            debug!("Created class: {class:?}");
                            spawned_maybe_class_setter.set(Some(Ok(class)));
                        } else {
                            error!("Failed to create class: {}", response.status());
                            let error_text = match response.text().await {
                                Ok(text) => text,
                                Err(error) => error.to_string(),
                            };
                            spawned_maybe_class_setter.set(Some(Err(format!(
                                "{} {}: {error_text}",
                                response.status(),
                                response.status_text(),
                            ))));
                        }
                    }
                    Err(error) => {
                        error!("Unable to set request body: {}", error);
                        spawned_maybe_class_setter.set(Some(Err(error.to_string())));
                    }
                }
            });

            is_creating.set(false);
        })
    };

    use_effect_with(maybe_class.clone(), move |_| {
        if let Some(toaster) = toaster.borrow() {
            if let Some(class_result) = (*maybe_class).borrow() {
                let (alert_type, title, body) = match class_result {
                    Ok(class) => {
                        if let Some(router) = maybe_router {
                            debug!("Navigating to class details page: {class:?}");
                            let class_id = class.id;
                            router.push(LeagueRoute::Class {
                                class_id,
                                page: crate::app::classes::ClassRoute::Details,
                            });
                        }
                        (
                            AlertType::Success,
                            "Class Created",
                            html!({ format!("Class \"{}\" created successfully", class.name) }),
                        )
                    }
                    Err(error) => {
                        (
                            AlertType::Danger,
                            "Error Creating Class",
                            html!(
                                <>
                                    <p>
                                        { "An error occurred while creating the class." }
                                    </p>
                                    <p>
                                        { error }
                                    </p>
                                </>
                            ),
                        )
                    }
                };

                toaster.toast(Toast {
                    title: title.to_string(),
                    r#type: alert_type,
                    timeout: Some(Duration::from_secs(5)),
                    body,
                    actions: Vec::new(),
                })
            }
        }
    });

    let html_content = html!(
        <PageContent title="Create Class">
            <Content>
                <Form {onsubmit}>
                    <FormGroup label="Class Name" required=true>
                        <TextInput
                            placeholder="Enter class name"
                            required=true
                            autofocus=true
                            value={(*class_name).clone()}
                            onchange={class_name_onchange}
                        />
                    </FormGroup>
                    <FormGroup label="Class Description">
                        <TextArea
                            placeholder="Enter class description"
                            rows=20
                            value={(*class_description).clone()}
                            onchange={class_description_onchange}
                        />
                    </FormGroup>
                    <ActionGroup>
                        <Button
                            variant={ButtonVariant::Primary}
                            label="Submit"
                            r#type={ButtonType::Submit}
                            icon={Icon::PlusCircle}
                            loading={*is_creating}
                        />
                    </ActionGroup>
                </Form>
            </Content>
        </PageContent>
    );

    Ok(html_content)
}
