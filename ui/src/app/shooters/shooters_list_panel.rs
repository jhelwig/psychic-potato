use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShootersListPanelProps {
    pub league_id: Uuid,
}

#[function_component(ShootersListPanel)]
pub fn shooters_create_panel(props: &ShootersListPanelProps) -> Html {
    html! {
        <div>
            <h2>
                { "Create Shooter" }
            </h2>
            <p>
                { format!("League ID: {}", props.league_id) }
            </p>
        </div>
    }
}
