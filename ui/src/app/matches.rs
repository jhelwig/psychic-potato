use uuid::Uuid;
use yew_nested_router::Target;

pub mod matches_panel;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum MatchesRoute {
    #[default]
    #[target(index)]
    Details,
    Create {
        league_id: Uuid,
    },
    Matches {
        league_id: Uuid,
    },
    Match {
        match_id: Uuid,
    },
}
