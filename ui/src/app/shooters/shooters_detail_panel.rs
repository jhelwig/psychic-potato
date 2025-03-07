use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShooterDetailPanelProps {
    pub league_id:  Uuid,
    pub shooter_id: Uuid,
}

#[function_component(ShooterDetailPanel)]
pub fn shooters_create_panel(props: &ShooterDetailPanelProps) -> Html {
    html! {
        <div>
            <h2>
                { format!("Shooter {}", props.shooter_id) }
            </h2>
            <p>
                { format!("League ID: {}", props.league_id) }
            </p>
        </div>
    }
}
