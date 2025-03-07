use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::{
    Target,
    components::Link,
    prelude::{
        Switch as RouterSwitch,
        *,
    },
};

pub mod admin;
pub mod leagues;
pub mod matches;
pub mod shooters;
pub mod sm_upload;

use crate::app::{
    admin::{
        AdminPanel,
        AdminRoute,
    },
    leagues::{
        LeagueRoute,
        LeaguesRoute,
        league_create_panel::CreateLeaguePanel,
        league_panel::LeaguePanel,
        leagues_panel::LeaguesPanel,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    #[target(index)]
    Index,
    Admin(AdminRoute),
    League {
        league_id: Uuid,
        #[target(nested, default)]
        page:      LeagueRoute,
    },
    Leagues(LeaguesRoute),
}

impl AppRoute {
    pub fn mapper_league(league_id: Uuid) -> Mapper<AppRoute, LeagueRoute> {
        let downwards = |app_route| {
            match app_route {
                AppRoute::League {
                    page,
                    ..
                } => Some(page),
                _ => None,
            }
        };
        let upwards = move |league_route| {
            AppRoute::League {
                league_id,
                page: league_route,
            }
        };

        Mapper::new(downwards, upwards)
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <AppPage>
                        <RouterSwitch<AppRoute> render={switch_app_route} />
                    </AppPage>
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn brand() -> Html {
    html! {
        <MastheadBrand>
            <Link<AppRoute> to={AppRoute::Index}>
                <Brand
                    src="images/logo.png"
                    alt="TCGC 600 yard league"
                    style="--pf-v5-c-brand--Height: 36px;"
                />
            </Link<AppRoute>>
        </MastheadBrand>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => {
            html! { <Index /> }
        }
        AppRoute::League {
            league_id,
            ..
        } => {
            html! {
                <Suspense fallback={format!("Loading league details...")}>
                    <Scope<AppRoute,LeagueRoute>
                        mapper={move |_| { AppRoute::mapper_league(league_id) }}
                    >
                        <LeaguePanel {league_id} />
                    </Scope<AppRoute,LeagueRoute>>
                </Suspense>
            }
        }
        AppRoute::Leagues(LeaguesRoute::Create) => {
            html!(<CreateLeaguePanel />)
        }
        AppRoute::Leagues(LeaguesRoute::Index) => {
            html! { <LeaguesPanel /> }
        }
        AppRoute::Admin(_) => {
            html! {
                <PageContent title="Admin">
                    <AdminPanel />
                </PageContent>
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
                    { leagues::leagues_nav_menu() }
                    { admin::admin_nav_menu() }
                </NavList>
            </Nav>
        </PageSidebar>
    };
    // TODO: Remember navbar open state across page reloads.
    let open = true;

    // TODO: Remember dark mode choice across page reloads.
    // TODO: Have On/Off/Auto choices for dark mode.
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
