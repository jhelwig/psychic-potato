use log::debug;
use patternfly_yew::prelude::*;
use shared_types::response::User;
use yew::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuthInfo {
    pub user:       Option<User>,
    pub auth_token: Option<String>,
}

#[function_component(AppLogin)]
pub fn app_login() -> Html {
    let Some(auth_info) = use_context::<AuthInfo>() else {
        return html!();
    };

    html!(
        if auth_info.user.is_some() {
            <UserInfo />
        } else {
            <LoginModalButton />
        }
    )
}

#[function_component(UserInfo)]
fn user_info() -> Html {
    let Some(auth_info) = use_context::<AuthInfo>() else {
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

    let onsubmit = {
        let form_state = form_state.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            match *form_state {
                ModalState::Login => {
                    // Login logic goes here
                    debug!("Logging in user with username: {}", &*username);
                }
                ModalState::Register => {
                    if *password == *confirm_password {
                        // Register logic goes here
                        debug!("Registering user with username: {}", &*username);
                    } else {
                        debug!("Passwords do not match");
                    }
                }
            }
        })
    };

    let footer = html! {
        <>
            <Button
                variant={ButtonVariant::Primary}
                label={match *form_state { ModalState::Login => "Log In", ModalState::Register => "Register" }}
                r#type={ButtonType::Submit}
                form="login-register-form"
            />
            <Button
                variant={ButtonVariant::Secondary}
                label="Cancel"
                r#type={ButtonType::Reset}
                onclick={cancel_onclick.clone()}
            />
        </>
    };
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
                />
            </FormGroup>
        </>
    );

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
