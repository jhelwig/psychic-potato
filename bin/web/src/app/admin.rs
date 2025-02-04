use leptos::{
    prelude::*,
    IntoView,
};
use leptos_router::components::Outlet;
use thaw::*;

pub mod assign_strings;
pub mod upload;

#[component]
pub fn AdminHome() -> impl IntoView {
    view! {
        <div>
            <h1>Admin Home</h1>
            <NavDrawer>
                <NavItem value="dashboard" icon=icondata::MdiMonitorDashboard href="/admin">
                    "Dashboard"
                </NavItem>
                <NavItem value="upload" icon=icondata::AiUploadOutlined href="/admin/upload">
                    "Upload"
                </NavItem>
                <NavItem value="assign_strings" icon=icondata::BsPersonFillCheck href="/admin/assign_strings">
                    "Assign Strings"
                </NavItem>
            </NavDrawer>
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
