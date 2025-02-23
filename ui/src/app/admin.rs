use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::{
    prelude::{
        Switch as RouterSwitch,
        *,
    },
    Target,
};

use crate::app::AppRoute;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AdminRoute {
    #[default]
    Dashboard,
    Upload,
}

pub fn admin_nav_menu() -> Html {
    let admin_check = true;

    if admin_check {
        html_nested! {
            <>
                <NavExpandable title="Admin">
                    <NavRouterItem<AppRoute> to={AppRoute::Admin(AdminRoute::Dashboard)}>
                        { "Dashboard" }
                    </NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Admin(AdminRoute::Upload)}>
                        { "Upload" }
                    </NavRouterItem<AppRoute>>
                </NavExpandable>
            </>
        }
    } else {
        html_nested!(
            <></>
        )
    }
}

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    html! {
        <Scope<AppRoute,AdminRoute> mapper={AppRoute::mapper_admin}>
            <RouterSwitch<AdminRoute> render={switch_admin_panel} />
        </Scope<AppRoute,AdminRoute>>
    }
}

fn switch_admin_panel(target: AdminRoute) -> Html {
    match target {
        AdminRoute::Dashboard => {
            html!(
                <Content>
                    { "Admin panel dashboard" }
                </Content>
            )
        }
        AdminRoute::Upload => {
            html!(
                <Content>
                    { "Admin panel upload" }
                </Content>
            )
        }
    }
}
