use uuid::Uuid;
use yew_nested_router::prelude::*;

pub mod shooters_create_panel;
pub mod shooters_detail_panel;
pub mod shooters_list_panel;
pub mod shooters_panel;

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum ShootersRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
    Detail {
        shooter_id: Uuid,
    },
}
