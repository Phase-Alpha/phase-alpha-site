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
                // Public Turnstile sitekey, read at request time and handed to
                // the browser here rather than compiled into the WASM bundle,
                // so it can be changed without a rebuild. `<head>` is never
                // hydrated, so there is no server/client mismatch to worry
                // about; turnstile.js reads the value back out of this tag.
                <meta
                    name="turnstile-sitekey"
                    content=std::env::var("TURNSTILE_SITE_KEY").unwrap_or_default()
                />
                <link rel="preconnect" href="https://challenges.cloudflare.com"/>
                // Loaded here rather than on the contact page so the helpers
                // survive client-side navigation. Both are deferred, which (unlike
                // async) guarantees they execute in document order, so
                // `onTurnstileLoad` is defined before api.js calls it.
                <script src="/turnstile.js" defer></script>
                <script
                    src="https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit&onload=onTurnstileLoad"
                    defer
                ></script>
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

        <Title text="Phase Alpha — custom software and design"/>

        // No `main` wrapper here: `Layout` renders the single `main` element,
        // along with the tab-bar, mode line and status bar around it. Nesting
        // one `main` inside another would be invalid.
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=home::HomePage/>
                <Route path=StaticSegment("services") view=services::Services/>
                <Route path=StaticSegment("contact") view=contact::Contact/>
                <Route path=StaticSegment("blog") view=blog::Blog/>
                <Route path=path!("blog/:post") view=blog::BlogPost/>
            </Routes>
        </Router>
    }
}
