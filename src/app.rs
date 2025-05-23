use crate::components::*;
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Create and provide the resource with the correct type for blog pages
    let posts = Resource::new(
        || (),
        |_| async move { get_posts("posts/".to_string()).await },
    );

    // Provide the resource to context
    provide_context(posts);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/phase-alpha-site.css"/>

        // sets the document title
        <Title text="Welcome to Phase Alpha"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=home::HomePage/>
                    <Route path=StaticSegment("services") view=services::Services/>
                    <Route path=StaticSegment("blog") view=blog::Blog/>
                    <Route path=path!("blog/:post") view=blog::BlogPost/>
                </Routes>
            </main>
        </Router>
    }
}
