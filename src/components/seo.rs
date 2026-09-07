use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// Canonical origin used to build absolute URLs for canonical links, Open
/// Graph/Twitter tags, and JSON-LD `url`/`image` fields.
pub const SITE_URL: &str = "https://www.phasealpha.io";

/// Fallback share image for pages that aren't a blog post. It's the site
/// logo (1500x1500) rather than a purpose-built 1200x630 share image, but a
/// square logo is a reasonable fallback and still renders fine in previews.
pub const DEFAULT_OG_IMAGE: &str = "/palogo.png";

/// Turns a site-relative path into an absolute `https://www.phasealpha.io/...` URL.
pub fn absolute_url(path: &str) -> String {
    format!("{SITE_URL}{path}")
}

/// Turns a site-relative or already-absolute image reference into an
/// absolute URL, since OG/Twitter/JSON-LD images should not be site-relative.
pub fn absolute_image(image: &str) -> String {
    if image.starts_with("http") {
        image.to_string()
    } else {
        absolute_url(image)
    }
}

/// Sets the per-page `<title>`, meta description, canonical link, Open Graph
/// and Twitter Card tags. Every routed page renders one of these so a single
/// shared `<Title>`/`<Meta>` doesn't leak across routes (Phase 1/2 of the SEO
/// task list).
#[component]
pub fn SeoMeta(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    /// Site-relative path this page is canonically reached at, e.g. `/services`.
    #[prop(into)] path: String,
    /// Site-relative or absolute share image. Defaults to the site logo.
    #[prop(optional)] image: Option<String>,
    /// `og:type` — `"website"` for ordinary pages, `"article"` for posts.
    #[prop(optional)] og_type: Option<String>,
) -> impl IntoView {
    let url = absolute_url(&path);
    let image_url = absolute_image(&image.unwrap_or_else(|| DEFAULT_OG_IMAGE.to_string()));
    let og_type = og_type.unwrap_or_else(|| "website".to_string());

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=url.clone()/>

        <Meta property="og:type" content=og_type/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=url/>
        <Meta property="og:image" content=image_url.clone()/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image_url/>
    }
}
