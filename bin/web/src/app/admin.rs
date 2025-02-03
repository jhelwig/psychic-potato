use leptos::{
    prelude::*,
    IntoView,
};
use leptos_router::components::Outlet;

pub mod upload;

#[component]
pub fn AdminHome() -> impl IntoView {
    view! {
        <div>
            <h1>Admin Home</h1>
            <Outlet />
        </div>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div>
            <h1>Dashboard</h1>
            <p>Welcome to the dashboard!</p>
        </div>
    }
}
