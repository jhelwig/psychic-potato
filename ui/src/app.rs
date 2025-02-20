use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::{
    components::*,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
    Target,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <RouterSwitch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn brand() -> Html {
    html! {
        <MastheadBrand>
            <Brand
                src="assets/images/logo.png"
                alt="Patternfly Logo"
                style="--pf-v5-c-branc--Height: 36px;"
            />
        </MastheadBrand>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => {
            html! {
                <AppPage>
                    <Index />
                </AppPage>
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
pub fn app_page(props: &PageProps) -> Html {
    let brand = brand();

    html! {
        <Page {brand}>
            { for props.children.iter() }
        </Page>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div>
            <p>
                { "Some example, placeholder content" }
            </p>
        </div>
    }
}
