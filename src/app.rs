use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::server_functions::posts::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let posts = create_resource(
        || (),
        |_| async { get_posts("public/posts/".to_string()).await },
    );
    provide_context(posts);

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="pkg/phase-alpha-site.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=home::HomePage/>
                    <Route path="/services" view=services::Services/>
                    <Route path="/blog" view=blog::Blog/>
                    <Route path="/blog/post/:post" view=blog::BlogPost/>
                </Routes>
            </main>
        </Router>
    }
}
