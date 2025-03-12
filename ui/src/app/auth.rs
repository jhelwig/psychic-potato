use std::{
    borrow::Borrow,
    rc::Rc,
    time::Duration,
};

use gloo_net::http::Request;
use log::{
    debug,
    error,
    info,
};
use patternfly_yew::prelude::*;
use shared_types::{
    request::RegisterUser,
    response::User,
};
use yew::{
    prelude::*,
    suspense::use_future,
};

use crate::api::perform_api_operation;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuthInfo {
    pub user: Option<User>,
}

impl Reducible for AuthInfo {
    type Action = Option<User>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AuthInfo {
            user: action,
        }
        .into()
    }
}

pub type AuthInfoContext = UseReducerHandle<AuthInfo>;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AuthInfoProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthInfoProvider)]
pub fn auth_info_provider(props: &AuthInfoProviderProps) -> Html {
    let auth_info = use_reducer(AuthInfo::default);

    html!(
        <ContextProvider<AuthInfoContext> context={auth_info}>
            { props.children.clone() }
        </ContextProvider<AuthInfoContext>>
    )
}

#[function_component(AppLogin)]
pub fn app_login() -> HtmlResult {
    let Some(auth_info) = use_context::<AuthInfoContext>() else {
        return Ok(html!({ "No AuthInfoContext" }));
    };

    let response = use_future(|| async move { fetch_current_user().await })?;

    if let Some(user) = &*response {
        info!("Logged in as {}", &user.username);
        auth_info.dispatch(Some(user.clone()));
    }

    Ok(html!(
        if auth_info.user.is_some() {
            <UserInfo />
        } else {
            <LoginModalButton />
        }
    ))
}

#[function_component(UserInfo)]
fn user_info() -> Html {
    let Some(auth_info) = use_context::<AuthInfoContext>() else {
        return html!();
    };
    let Some(user) = auth_info.user.as_ref() else {
        return html!();
    };

    html!({ format!("Welcome, {}!", user.username) })
}

#[function_component(LoginModalButton)]
fn login_modal_button() -> Html {
    let Some(backdropper) = use_backdrop() else {
        return html!();
    };

    let onclick = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            backdropper.open(Backdrop::new(
                html!(<LogInOrRegisterPanel backdropper={backdropper.clone()} />),
            ));
        })
    };

    html! {
        <>
            <Button variant={ButtonVariant::Primary} {onclick}>
                { "Log In" }
            </Button>
        </>
    }
}

#[remain::sorted]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ModalState {
    Login,
    Register,
}

#[derive(Clone, PartialEq, Properties)]
struct LogInOrRegisterPanelProps {
    pub backdropper: Backdropper,
}

#[function_component(LogInOrRegisterPanel)]
fn log_in_or_register_panel(props: &LogInOrRegisterPanelProps) -> Html {
    let backdropper = props.backdropper.clone();
    let username = use_state_eq(String::new);
    let password = use_state_eq(String::new);
    let confirm_password = use_state_eq(String::new);
    let is_password_confirmation_match = use_state(|| true);
    let is_submitting = use_state_eq(|| false);
    let maybe_user: UseStateHandle<Option<Result<User, String>>> = use_state_eq(|| None);
    let toaster = use_toaster();
    let auth_info = use_context::<AuthInfoContext>();

    let cancel_onclick = {
        let backdropper = backdropper.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            backdropper.close();
        })
    };

    let form_state = use_state(|| ModalState::Login);

    let tabs_onselect = {
        // let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();

        use_callback(form_state.clone(), move |new_form_state, form_state| {
            password.set(String::new());
            confirm_password.set(String::new());

            form_state.set(new_form_state);
        })
    };

    let username_onchange = use_callback(username.clone(), |new_username, username| {
        username.set(new_username);
    });
    let password_onchange = use_callback(password.clone(), |new_password, password| {
        password.set(new_password);
    });
    let confirm_password_onchange =
        use_callback(confirm_password.clone(), |new_confirm_password, confirm_password| {
            confirm_password.set(new_confirm_password);
        });

    let effect_is_password_confirmation_match = is_password_confirmation_match.clone();
    use_effect_with(
        (confirm_password.clone(), password.clone()),
        move |(confirm_password, password)| {
            effect_is_password_confirmation_match.set(*password == *confirm_password);
        },
    );

    let onsubmit = {
        let form_state = form_state.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let is_submitting = is_submitting.clone();
        let maybe_user = maybe_user.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_submitting.set(true);

            match *form_state {
                ModalState::Login => {
                    // Login logic goes here
                    debug!("Logging in user with username: {}", &*username);
                    let login_payload = shared_types::request::Login {
                        username: (*username).clone(),
                        password: (*password).clone(),
                    };
                    let spawned_maybe_user_setter = maybe_user.setter();
                    wasm_bindgen_futures::spawn_local(perform_api_operation(
                        "/api/user/login".to_string(),
                        login_payload,
                        spawned_maybe_user_setter,
                    ));
                }
                ModalState::Register => {
                    if *password == *confirm_password {
                        // Register logic goes here
                        debug!("Registering user with username: {}", &*username);
                        let register_payload = RegisterUser {
                            username: (*username).clone(),
                            password: (*password).clone(),
                        };
                        let spawned_maybe_user_setter = maybe_user.setter();
                        wasm_bindgen_futures::spawn_local(perform_api_operation(
                            "/api/user/register".to_string(),
                            register_payload,
                            spawned_maybe_user_setter,
                        ));
                    } else {
                        debug!("Passwords do not match");
                    }
                }
            }

            is_submitting.set(false);
        })
    };

    use_effect_with(maybe_user.clone(), move |_| {
        let (alert_type, title, body) = if let Some(user_result) = (*maybe_user).borrow() {
            match user_result {
                Ok(user) => {
                    if let Some(auth_info) = auth_info.borrow() {
                        auth_info.dispatch(Some(user.clone()));
                    }
                    (
                        AlertType::Success,
                        "Registration Successful",
                        html!({ format!("Welcome, {}", user.username) }),
                    )
                }
                Err(error) => {
                    (
                        AlertType::Danger,
                        "Registration Failed",
                        html!(
                            <>
                                <p>
                                    { "An error occurred while registering." }
                                </p>
                                <p>
                                    { error }
                                </p>
                            </>
                        ),
                    )
                }
            }
        } else {
            return;
        };
        if let Some(toaster) = toaster.borrow() {
            toaster.toast(Toast {
                title: title.to_string(),
                r#type: alert_type,
                timeout: Some(Duration::from_secs(5)),
                body,
                actions: Vec::new(),
            })
        }
    });

    let log_in_form = html_nested!(
        <>
            <FormGroup label="Username">
                <TextInput
                    required=true
                    name="username"
                    value={(*username).clone()}
                    autofocus=true
                    onchange={username_onchange.clone()}
                />
            </FormGroup>
            <FormGroup label="Password">
                <TextInput
                    required=true
                    name="password"
                    value={(*password).clone()}
                    r#type={TextInputType::Password}
                    onchange={password_onchange.clone()}
                />
            </FormGroup>
        </>
    );
    let register_form = html_nested!(
        <>
            <FormGroup label="Username">
                <TextInput
                    required=true
                    name="username"
                    value={(*username).clone()}
                    autofocus=true
                    onchange={username_onchange.clone()}
                />
            </FormGroup>
            <FormGroup label="Password">
                <TextInput
                    required=true
                    name="password"
                    value={(*password).clone()}
                    r#type={TextInputType::Password}
                    onchange={password_onchange.clone()}
                />
            </FormGroup>
            <FormGroup label="Confirm Password">
                <TextInput
                    required=true
                    name="confirm_password"
                    value={(*confirm_password).clone()}
                    r#type={TextInputType::Password}
                    onchange={confirm_password_onchange.clone()}
                    state={if *is_password_confirmation_match.clone() {InputState::Success} else {InputState::Error}}
                />
            </FormGroup>
        </>
    );
    let footer = html! {
        <>
            <Button
                variant={ButtonVariant::Primary}
                label={match *form_state { ModalState::Login => "Log In", ModalState::Register => "Register" }}
                r#type={ButtonType::Submit}
                form="login-register-form"
                loading={*(is_submitting.clone())}
            />
            <Button
                variant={ButtonVariant::Secondary}
                label="Cancel"
                r#type={ButtonType::Reset}
                onclick={cancel_onclick.clone()}
            />
        </>
    };

    html!(
        <Bullseye>
            <Modal variant={ModalVariant::Large} {footer}>
                <Tabs<ModalState>
                    detached=true
                    onselect={tabs_onselect.clone()}
                    selected={*form_state}
                    r#box=true
                >
                    <Tab<ModalState> index={ModalState::Login} title="Log In" />
                    <Tab<ModalState> index={ModalState::Register} title="Register" />
                </Tabs<ModalState>>
                <Form {onsubmit} id="login-register-form">
                    if *form_state == ModalState::Login {
                        { log_in_form }
                    } else {
                        { register_form }
                    }
                </Form>
            </Modal>
        </Bullseye>
    )
}

async fn fetch_current_user() -> Option<User> {
    let response = match Request::get("/api/user").send().await {
        Ok(resp) => resp,
        Err(error) => {
            error!("Error fetching current user: {error}");
            return None;
        }
    };
    let user: User = if response.ok() {
        match response.json().await {
            Ok(user) => user,
            Err(error) => {
                error!("Unable to parse response: {error}");
                return None;
            }
        }
    } else {
        let response_text = match response.text().await {
            Ok(t) => t,
            Err(error) => {
                error!("Error fetching current user: {error}");
                return None;
            }
        };
        error!("Failed to fetch current user ({}): {response_text}", response.status());
        return None;
    };

    Some(user)
}
