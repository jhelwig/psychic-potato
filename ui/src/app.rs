use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::{
    prelude::{
        Switch as RouterSwitch,
        *,
    },
    Target,
};

mod admin;

use admin::{
    AdminPanel,
    AdminRoute,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
    Admin(AdminRoute),
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
                src="images/logo.png"
                alt="Patternfly Logo"
                style="--pf-v5-c-brand--Height: 36px;"
            />
        </MastheadBrand>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Admin(_) => {
            html! {
                <AppPage>
                    <PageContent title="Admin">
                        <AdminPanel />
                    </PageContent>
                </AppPage>
            }
        }
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

    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavRouterItem<AppRoute> to={AppRoute::default()}>
                        { "Home" }
                    </NavRouterItem<AppRoute>>
                    { admin::admin_nav_menu() }
                </NavList>
            </Nav>
        </PageSidebar>
    };
    let open = false;

    let darkmode = use_state_eq(|| {
        gloo_utils::window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|m| m.matches())
            .unwrap_or_default()
    });
    use_effect_with(*darkmode, |state| {
        match state {
            true => gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
            false => gloo_utils::document_element().set_class_name(""),
        }
    });

    let onthemeswitch = use_callback(darkmode.setter(), |state, setter| setter.set(state));

    let tools = html! {
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch
                            checked={*darkmode}
                            onchange={onthemeswitch}
                            label="Dark Theme"
                        />
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    };

    html! {
        <Page {brand} {sidebar} {tools} {open}>
            { for props.children.iter() }
        </Page>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <PageContent title="Dashboard">
            <p>
                { "Some example, placeholder content" }
            </p>
        </PageContent>
    }
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct PageContentProps {
    pub title:    AttrValue,
    #[prop_or_default]
    pub subtitle: Children,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageContent)]
pub fn page_content(props: &PageContentProps) -> Html {
    html! {
        <PageSectionGroup>
            <PageSection
                r#type={PageSectionType::Default}
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Content>
                    <Title size={Size::XXXXLarge}>
                        { props.title.clone() }
                    </Title>
                    { for props.subtitle.iter() }
                </Content>
            </PageSection>
            { for props.children.iter().map(|child| html!(<PageSection>{ child }</PageSection>)) }
        </PageSectionGroup>
    }
}
