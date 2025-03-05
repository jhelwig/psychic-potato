use yew_nested_router::prelude::*;

pub mod match_details_panel;
pub mod matches_create_panel;
pub mod matches_list_panel;
pub mod matches_panel;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesRoute {
    #[default]
    #[target(index)]
    Index,
    Create,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum MatchRoute {
    #[default]
    #[target(index)]
    Details,
}
