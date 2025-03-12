use std::{
    borrow::Borrow,
    time::Duration,
};

use anyhow::Result;
use gloo_net::http::Request;
use log::{
    debug,
    error,
};
use patternfly_yew::prelude::*;
use shared_types::{
    request::LeagueOperation,
    response::League,
};
use yew::prelude::*;
use yew_nested_router::prelude::*;

use crate::app::{
    AppRoute,
    PageContent,
    leagues::LeagueRoute,
};

#[function_component(CreateLeaguePanel)]
pub fn create_league_panel() -> HtmlResult {
    let league_name = use_state_eq(String::new);
    let is_creating = use_state_eq(|| false);
    let maybe_league: UseStateHandle<Option<Result<League, String>>> = use_state_eq(|| None);
    let maybe_router = use_router::<AppRoute>();
    let toaster = use_toaster();

    let onchange = use_callback(league_name.clone(), |new_league_name, league_name| {
        league_name.set(new_league_name);
    });

    let onsubmit = {
        let league_name = league_name.clone();
        let is_creating = is_creating.setter();
        let maybe_league_setter = maybe_league.setter();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_creating.set(true);

            // Create league using league_name
            let league_operation = LeagueOperation::Create {
                league_name: (*league_name).clone(),
            };

            let spawned_league_name = league_name.clone();
            let spawned_maybe_league_setter = maybe_league_setter.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = match Request::post("/api/league/operation").json(&league_operation)
                {
                    Ok(req) => req.send().await,
                    Err(error) => {
                        error!("Unable to set request body: {}", error);
                        spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                        return;
                    }
                };
                match response {
                    Ok(response) => {
                        if response.ok() {
                            let league: League = match response.json().await {
                                Ok(league) => {
                                    spawned_league_name.set(String::new());
                                    league
                                }
                                Err(error) => {
                                    error!("Unable to parse response: {}", error);
                                    spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                                    return;
                                }
                            };
                            debug!("Created league: {league:?}");
                            spawned_maybe_league_setter.set(Some(Ok(league)));
                        } else {
                            error!("Failed to create league: {}", response.status());
                            let error_text = match response.text().await {
                                Ok(text) => text,
                                Err(error) => error.to_string(),
                            };
                            spawned_maybe_league_setter.set(Some(Err(format!(
                                "{} {}: {error_text}",
                                response.status(),
                                response.status_text()
                            ))));
                        }
                    }
                    Err(error) => {
                        error!("Error creating league: {}", error);
                        spawned_maybe_league_setter.set(Some(Err(error.to_string())));
                    }
                }
            });

            is_creating.set(false);
        })
    };

    use_effect_with(maybe_league.clone(), move |_| {
        if let Some(toaster) = toaster.borrow() {
            if let Some(league_result) = (*maybe_league).borrow() {
                let (alert_type, title, body) = match league_result {
                    Ok(league) => {
                        if let Some(router) = maybe_router {
                            debug!("Navigating to league details page: {league:?}");
                            let league_id = league.id;
                            router.push(AppRoute::League {
                                league_id,
                                page: LeagueRoute::Details,
                            });
                        }

                        (
                            AlertType::Success,
                            "League Created",
                            html!(
                                { format!("League \"{}\" created successfully.", league.name.clone()) }
                            ),
                        )
                    }
                    Err(error) => {
                        (
                            AlertType::Danger,
                            "Error Creating League",
                            html!(
                                <>
                                    <p>
                                        { "An error occurred while creating the league." }
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
                });
            }
        }
    });

    let html_content = html!(
        <PageContent title="Create League">
            <Content>
                <Form {onsubmit}>
                    <FormGroup label="Legue Name" required=true>
                        <TextInput
                            placeholder="Enter league name"
                            required=true
                            autofocus=true
                            value={(*league_name).clone()}
                            {onchange}
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
