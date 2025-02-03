use leptos::prelude::*;
use leptos_meta::{
    provide_meta_context,
    MetaTags,
    Stylesheet,
    Title,
};
use leptos_router::{
    components::{
        ParentRoute,
        Route,
        Router,
        Routes,
    },
    StaticSegment,
};
use thaw::*;

mod admin;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use thaw::ssr::SSRMountStyleProvider;
    view! {
        <SSRMountStyleProvider>
            <ShellInner options=options.clone() />
        </SSRMountStyleProvider>
    }
}

#[cfg(not(feature = "ssr"))]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! { <ShellInner options=options.clone() /> }
}

#[component]
pub fn ShellInner(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web.css" />
        <Stylesheet
            id="remix-icon"
            href="https://cdn.jsdelivr.net/npm/remixicon@4.6.0/fonts/remixicon.css"
        />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <ConfigProvider>
            <ToasterProvider>
                <nav>
                    <a href="/">Home</a>
                    <a href="/admin">Admin</a>
                </nav>
                <Router>
                    <main>
                        <MessageBar intent=MessageBarIntent::Warning>
                            <MessageBarBody>
                                <MessageBarTitle>"Under Construction"</MessageBarTitle>
                                "This site is still in development. Expect broken things. You have been warned."
                            </MessageBarBody>
                        </MessageBar>
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage />
                            <ParentRoute path=StaticSegment("admin") view=admin::AdminHome>
                                <Route
                                    path=StaticSegment("upload")
                                    view=admin::upload::ShotMarkerCsvUpload
                                />
                                <Route path=StaticSegment("") view=admin::Dashboard />
                            </ParentRoute>
                        </Routes>
                    </main>
                </Router>
            </ToasterProvider>
        </ConfigProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
