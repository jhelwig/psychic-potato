use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::app::{
    PageContent,
    leagues::league_list::LeagueList,
};

#[function_component(LeaguesPanel)]
pub fn leagues_panel() -> Html {
    html! {
        <PageContent title="Leagues">
            <Content>
                <Suspense fallback={html!({"Loading..."})}>
                    <LeagueList />
                </Suspense>
            </Content>
        </PageContent>
    }
}
