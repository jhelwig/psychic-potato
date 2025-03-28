use std::rc::Rc;

use log::{
    debug,
    error,
    info,
    warn,
};
use patternfly_yew::prelude::*;
use popper_rs::prelude::{
    State as PopperState,
    *,
};
use shared_types::{
    request::{
        ShooterOperation,
        ShotStringOperation,
    },
    response::{
        League,
        Match,
        Shooter,
        ShotMarkerShotString,
    },
};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::{
    use_click_away,
    use_event_with_window,
};

use crate::{
    api::perform_api_operation,
    app::shooters::fetch_shooter_suggestions,
};

const SHOOTER_SEARCH_INPUT_ID: &str = "shooter-search-input";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShooterAutocompleteProps {
    pub league:         Rc<League>,
    pub match_object:   Rc<Match>,
    pub shot_string:    Rc<ShotMarkerShotString>,
    pub initial_search: Option<String>,
}

#[function_component(ShooterAutocomplete)]
pub fn shooter_autocomplete(props: &ShooterAutocompleteProps) -> HtmlResult {
    let league_id = props.league.id;
    let match_id = props.match_object.id;
    let shot_string_id = props.shot_string.id;

    let search_text = use_state(String::new);
    let value = use_state(|| None::<Shooter>);
    let onclear =
        use_callback((search_text.setter(), value.setter()), |_, (search_text, value)| {
            search_text.set(String::new());
            value.set(None)
        });
    let possible_values: UseStateHandle<Vec<Shooter>> = use_state_eq(Vec::new);

    let away_ref = use_node_ref();
    let input_ref = use_node_ref();
    let menu_ref = use_node_ref();

    let state = use_state_eq(PopperState::default);
    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    let autocomplete_open = use_state_eq(|| false);

    let hint = use_state_eq(|| None::<AttrValue>);

    let onchange = {
        use_callback(
            (
                search_text.clone(),
                hint.setter(),
                autocomplete_open.setter(),
                possible_values.clone(),
            ),
            |new: String, (search_text, hint, autocomplete_open, possible_values)| {
                autocomplete_open.set(!new.is_empty());

                search_text.set(new.clone());

                let spawned_possible_values = possible_values.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_shooter_suggestions(&new).await {
                        Ok(values) => {
                            spawned_possible_values.set(values.clone());
                        }
                        Err(error) => {
                            error!("Error fetching shooter suggestions: {}", error);
                        }
                    }
                });

                if let Some(first_suggestion) = possible_values.first() {
                    hint.set(Some(first_suggestion.name.clone().into()));
                } else {
                    hint.set(None);
                }
            },
        )
    };

    // Keyboard handling
    {
        let search_text = search_text.clone();
        let value = value.clone();
        let hint = hint.clone();
        let autocomplete_open = autocomplete_open.clone();
        let input_ref = input_ref.clone();
        let menu_ref = menu_ref.clone();

        use_event_with_window("keydown", move |e: KeyboardEvent| {
            let in_input = input_ref.get().as_deref() == e.target().as_ref();

            match e.key().as_str() {
                "Tab" | "ArrowRight" if in_input => {
                    if let Some(hint_value) = &*hint {
                        if *autocomplete_open {
                            e.prevent_default();
                        }
                        hint.set(None);
                        search_text.set(hint_value.to_string());
                        autocomplete_open.set(false);
                        input_ref.focus();
                    }
                }
                "ArrowUp" | "ArrowDown" if in_input => {
                    if let Some(first) = menu_ref
                        .cast::<web_sys::HtmlElement>()
                        .and_then(|ele| {
                            ele.query_selector("li > button:not(:disabled)").ok().flatten()
                        })
                        .and_then(|ele| ele.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let _ = first.focus();
                    }
                    e.prevent_default();
                }
                "Escape" => {
                    autocomplete_open.set(false);
                    input_ref.focus();
                }
                _ => {}
            }
        });
    }
    let maybe_new_shooter: UseStateHandle<Option<Result<Shooter, String>>> = use_state(|| None);

    // The autocomplete menu
    let autocomplete = {
        let search_text = search_text.clone();
        let setter = value.setter();
        let autocomplete_open = autocomplete_open.setter();
        let maybe_new_shooter = maybe_new_shooter.clone();

        let create_onclick = {
            let search_text = search_text.clone();
            let setter = setter.clone();
            let autocomplete_open = autocomplete_open.clone();
            let input_ref = input_ref.clone();
            let maybe_new_shooter_setter = maybe_new_shooter.setter();

            Callback::from(move |_| {
                let maybe_new_shooter_setter = maybe_new_shooter_setter.clone();
                // Create new shooter & assign
                let shooter_name = (*search_text).clone();
                info!("Creating new shooter: {}", &shooter_name);
                let operation = ShooterOperation::Create {
                    name:             shooter_name,
                    default_class_id: None,
                };
                wasm_bindgen_futures::spawn_local(perform_api_operation(
                    "/api/shooter/operation".to_string(),
                    operation,
                    Some(maybe_new_shooter_setter),
                ));

                match &*maybe_new_shooter {
                    Some(Ok(shooter)) => {
                        debug!("Created new shooter: {shooter:?}");
                        setter.set(Some(shooter.clone()));
                        autocomplete_open.set(false);
                    }
                    Some(Err(error)) => {
                        error!("Error creating new shooter: {error}");
                        setter.set(None);
                    }
                    None => {
                        warn!("Unable to create new shooter.");
                    }
                }
            })
        };
        let show_create_option = !search_text.trim().is_empty();

        html!(
            <Menu
                r#ref={menu_ref.clone()}
                style={&state.styles.popper.extend_with("z-index", "1000")}
            >
                { if show_create_option {
                    Some(html_nested!(<MenuAction onclick={create_onclick}>
                        { format!("Create new shooter: {}", &*search_text) }
                    </MenuAction>))
                } else { None} }
                { for possible_values.iter().map(|possible_value| {
                    let onclick = {
                        let possible_value = possible_value.clone();
                        let search_text = search_text.clone();
                        let setter = setter.clone();
                        let autocomplete_open = autocomplete_open.clone();
                        let input_ref = input_ref.clone();

                        Callback::from(move |_| {
                            let string_operation = ShotStringOperation::SetShooter {
                                id: shot_string_id,
                                shooter_id: possible_value.id,
                            };
                            wasm_bindgen_futures::spawn_local(perform_api_operation(format!("/api/league/{league_id}/match/{match_id}/string/operation"), string_operation, None::<UseStateSetter<Option<Result<String, String>>>>));
                            setter.set(Some(possible_value.clone()));
                            search_text.set(possible_value.name.clone());
                            autocomplete_open.set(false);
                            input_ref.focus();
                        })
                    };

                    html_nested!(
                        <MenuAction {onclick}>{possible_value.name.clone()}</MenuAction>
                    )
                }) }
            </Menu>
        )
    };

    // Close the autocomplete menu when clicking outside of it
    {
        let autocomplete_open = autocomplete_open.clone();
        use_click_away(away_ref.clone(), move |_: Event| autocomplete_open.set(false));
    }

    Ok(html!(
        <>
            <div>
                { format!("Shooter: {:?}", &*value) }
            </div>
            <div ref={away_ref} style="display: block;">
                <SearchInput
                    id={SHOOTER_SEARCH_INPUT_ID}
                    inner_ref={input_ref.clone()}
                    placeholder="Find by name"
                    value={(*search_text).clone()}
                    {onchange}
                    {onclear}
                    hint={(*hint).clone()}
                />
                <PortalToPopper
                    popper={yew::props!(PopperProperties {
                            target: input_ref.clone(),
                            content: menu_ref.clone(),
                            placement: Placement::Bottom,
                            visible: *autocomplete_open,
                            modifiers: vec![
                                Modifier::SameWidth(Default::default()),
                            ],
                            onstatechange,
                        })}
                    append_to={gloo_utils::document().get_element_by_id(SHOOTER_SEARCH_INPUT_ID)}
                >
                    { autocomplete }
                </PortalToPopper>
            </div>
        </>
    ))
}
