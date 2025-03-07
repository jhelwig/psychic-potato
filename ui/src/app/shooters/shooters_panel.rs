use std::rc::Rc;

use shared_types::response::League;
use uuid::Uuid;
use yew::prelude::*;
use yew_nested_router::prelude::{
    Switch as RouterSwitch,
    *,
};

use crate::app::{
    leagues::LeagueRoute,
    shooters::{
        ShootersRoute,
        shooters_create_panel::ShootersCreatePanel,
        shooters_detail_panel::ShooterDetailPanel,
        shooters_list_panel::ShootersListPanel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShootersPanelProps {
    pub league: Rc<League>,
}

#[function_component(ShootersPanel)]
pub fn shooters_panel(props: &ShootersPanelProps) -> Html {
    let league_id = props.league.id;

    html!(
        <>
            <Scope<LeagueRoute,ShootersRoute> mapper={LeagueRoute::mapper_shooters}>
                <RouterSwitch<ShootersRoute>
                    render={move |target| { switch_shooters_panel(league_id, target)}}
                />
            </Scope<LeagueRoute,ShootersRoute>>
        </>
    )
}

fn switch_shooters_panel(league_id: Uuid, target: ShootersRoute) -> Html {
    let route = match target {
        ShootersRoute::Index => html!(<ShootersListPanel {league_id} />),
        ShootersRoute::Create => html!(<ShootersCreatePanel {league_id} />),
        ShootersRoute::Detail {
            shooter_id,
        } => html!(<ShooterDetailPanel {league_id} {shooter_id} />),
    };

    html!({ route })
}
