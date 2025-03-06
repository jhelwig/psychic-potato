use std::borrow::Borrow;

use anyhow::Result;
use chrono::{
    NaiveDate,
    Utc,
};
use gloo_net::http::Request;
use log::{
    debug,
    error,
};
use patternfly_yew::prelude::*;
use shared_types::{
    request::MatchOperation,
    response::Match,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::prelude::*;

use crate::app::{
    PageContent,
    leagues::LeagueRoute,
    matches::MatchRoute,
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct MatchesCreatePanelProps {
    pub league_id: Uuid,
}

#[function_component(MatchesCreatePanel)]
pub fn matches_create_panel(props: &MatchesCreatePanelProps) -> HtmlResult {
    let league_id = props.league_id;
    let match_name = use_state_eq(String::new);
    let match_date = use_state_eq(|| None);
    let is_creating = use_state_eq(|| false);
    let maybe_match: UseStateHandle<Option<Result<Match, String>>> = use_state_eq(|| None);
    let datepicker_state = use_state_eq(|| None);
    let maybe_router = use_router::<LeagueRoute>();
    let toaster = use_toaster();

    let match_name_onchange = use_callback(match_name.clone(), |new_match_name, match_name| {
        match_name.set(new_match_name);
    });

    let datepicker_onchange =
        use_callback(datepicker_state.clone(), |new_datepicker_state, datepicker_state| {
            datepicker_state.set(Some(new_datepicker_state));
        });

    let onsubmit = {
        let match_name = match_name.clone();
        let is_creating = is_creating.setter();
        let maybe_match_setter = maybe_match.setter();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_creating.set(true);

            // Create match using match_name
            let match_operation = MatchOperation::Create {
                name:       (*match_name).clone(),
                event_date: (*match_date).unwrap_or_else(|| Utc::now().naive_local().date()),
            };
            let spawned_match_name = match_name.clone();
            let spawned_maybe_match_setter = maybe_match_setter.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    match Request::post(&format!("/api/league/{league_id}/match/operation"))
                        .json(&match_operation)
                    {
                        Ok(req) => req.send().await,
                        Err(error) => {
                            error!("Unable to set request body: {}", error);
                            spawned_maybe_match_setter.set(Some(Err(error.to_string())));
                            return;
                        }
                    };
                match response {
                    Ok(response) => {
                        if response.ok() {
                            let match_object: Match = match response.json().await {
                                Ok(match_object) => {
                                    spawned_match_name.set(String::new());
                                    match_object
                                }
                                Err(error) => {
                                    error!("Unable to parse response: {}", error);
                                    spawned_maybe_match_setter.set(Some(Err(error.to_string())));
                                    return;
                                }
                            };
                            debug!("Created match: {match_object:?}");
                            spawned_maybe_match_setter.set(Some(Ok(match_object)));
                        } else {
                            error!("Failed to create match: {}", response.status());
                            let error_text = match response.text().await {
                                Ok(text) => text,
                                Err(error) => error.to_string(),
                            };
                            spawned_maybe_match_setter.set(Some(Err(format!(
                                "{} {}: {error_text}",
                                response.status(),
                                response.status_text(),
                            ))));
                        }
                    }
                    Err(error) => {
                        error!("Error creating match: {}", error);
                        spawned_maybe_match_setter.set(Some(Err(error.to_string())));
                    }
                }
            });

            is_creating.set(false);
        })
    };

    use_effect_with(maybe_match.clone(), move |_| {
        if let Some(toaster) = toaster.borrow() {
            if let Some(match_result) = (*maybe_match).borrow() {
                let (alert_type, title, body) = match match_result {
                    Ok(match_object) => {
                        if let Some(router) = maybe_router {
                            debug!("Navigating to match details page: {match_object:?}");
                            let match_id = match_object.id;
                            router.push(LeagueRoute::Match {
                                match_id,
                                page: MatchRoute::Details,
                            });
                        }

                        (
                            AlertType::Success,
                            "Match Created",
                            html!(
                                { format!(
                                    "Match \"{}\" has been created successfully.",
                                    match_object.name.clone()
                                ) }
                            ),
                        )
                    }
                    Err(error) => {
                        (
                            AlertType::Danger,
                            "Error Creating Match",
                            html!(
                                <>
                                    <p>
                                        { "An error occurred while creating the match." }
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
                    timeout: None,
                    body,
                    actions: Vec::new(),
                })
            }
        }
    });

    let html_content = html!(
        <PageContent title="Create Match">
            <Content>
                <Form {onsubmit}>
                    <FormGroup label="Match Name" required=true>
                        <TextInput
                            placeholder="Enter match name"
                            required=true
                            autofocus=true
                            value={(*match_name).clone()}
                            onchange={match_name_onchange}
                        />
                    </FormGroup>
                    <FormGroup label="Event Date">
                        <DatePicker onchange={datepicker_onchange} value={*datepicker_state} />
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
