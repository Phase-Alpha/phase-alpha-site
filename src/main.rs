/// Site's static routes, used to build `sitemap.xml` alongside the blog posts.
#[cfg(feature = "ssr")]
const STATIC_ROUTES: [&str; 4] = ["", "services", "contact", "blog"];

/// Generates `sitemap.xml` from the static routes plus every post's current
/// slug, so search engines can discover posts without depending on the blog
/// index being crawled and followed first.
#[cfg(feature = "ssr")]
async fn sitemap_xml() -> impl axum::response::IntoResponse {
    use phase_alpha_site::server_functions::posts::{order_posts, read_markdown_files};

    const SITE_URL: &str = "https://www.phasealpha.io";
    let posts = order_posts(read_markdown_files("posts/".to_string()));

    let mut body = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for route in STATIC_ROUTES {
        body.push_str(&format!("  <url><loc>{SITE_URL}/{route}</loc></url>\n"));
    }
    for post in &posts {
        body.push_str(&format!(
            "  <url><loc>{SITE_URL}/blog/{}</loc><lastmod>{}</lastmod></url>\n",
            post.meta_data.create_href(),
            post.meta_data.date,
        ));
    }
    body.push_str("</urlset>\n");

    ([(axum::http::header::CONTENT_TYPE, "application/xml")], body)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::{post, get}};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use phase_alpha_site::app::*;
    use phase_alpha_site::server_functions::url_shorten::{redirect, shorten_url};

    // Server functions load this themselves, but `shell` needs the Turnstile
    // sitekey while rendering, so pull the file in once up front.
    dotenv::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/shorten_url", post(shorten_url))
        .route("/short/{uuid}", get(redirect))
        .route("/sitemap.xml", get(sitemap_xml))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
