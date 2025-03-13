use std::{
    borrow::Borrow,
    time::Duration,
};

use anyhow::Result;
use log::debug;
use patternfly_yew::prelude::*;
use shared_types::{
    request::LeagueOperation,
    response::League,
};
use yew::prelude::*;
use yew_nested_router::prelude::*;

use crate::{
    api::perform_api_operation,
    app::{
        AppRoute,
        PageContent,
        leagues::LeagueRoute,
    },
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
        let maybe_league = maybe_league.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_creating.set(true);

            // Create league using league_name
            let league_operation = LeagueOperation::Create {
                league_name: (*league_name).clone(),
            };

            let spawned_league_name = league_name.clone();
            let spawned_maybe_league_setter = maybe_league.setter();
            wasm_bindgen_futures::spawn_local(perform_api_operation(
                "/api/league/operation".to_string(),
                league_operation,
                Some(spawned_maybe_league_setter.clone()),
            ));
            is_creating.set(false);
            if matches!(&*maybe_league, Some(Ok(_))) {
                spawned_league_name.set(String::new());
            }
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
