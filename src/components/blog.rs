use crate::components::layout::Layout;
use crate::components::seo::{absolute_image, absolute_url, SeoMeta};
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_meta::Script;
use leptos_router::hooks::use_params_map;

/// How many related posts to show at the bottom of a post.
const RELATED_COUNT: usize = 3;

/// If `requested` doesn't match any post's current `slug` but does match a
/// post's pre-slug `legacy_href` (i.e. it's an old indexed/bookmarked URL),
/// tell the browser to permanently redirect to the current slug. No-op
/// (and always `false`) on the client build, since by the time hydration
/// runs on a legacy URL the server should already have redirected it.
#[cfg(feature = "ssr")]
fn redirect_legacy_post(requested: &str, posts: &[Post]) -> bool {
    use leptos_axum::ResponseOptions;

    let Some(post) = posts
        .iter()
        .find(|p| p.meta_data.legacy_href() == requested && p.meta_data.create_href() != requested)
    else {
        return false;
    };

    let response_opts = expect_context::<ResponseOptions>();
    response_opts.set_status(axum::http::StatusCode::MOVED_PERMANENTLY);
    if let Ok(location) =
        axum::http::HeaderValue::from_str(&format!("/blog/{}", post.meta_data.create_href()))
    {
        response_opts.insert_header(axum::http::header::LOCATION, location);
    }
    true
}

#[cfg(not(feature = "ssr"))]
fn redirect_legacy_post(_requested: &str, _posts: &[Post]) -> bool {
    false
}

#[component]
pub fn Blog() -> impl IntoView {
    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find posts resource");

    // Mirrors the mockup's `13 entries` status bar segment.
    let count = Signal::derive(move || {
        posts
            .get()
            .and_then(Result::ok)
            .map(|posts| format!("{} entries", posts.len()))
            .unwrap_or_default()
    });

    // Note the single `ul`. The previous version nested `ul.post-list` inside a
    // bare `ul`, which is invalid markup and rendered both.
    let rows = move || {
        posts.and_then(|posts| {
            if posts.is_empty() {
                return view! { <p class="card__desc">"No posts yet."</p> }.into_any();
            }
            posts
                .iter()
                .map(|post| {
                    let href = format!("/blog/{}", post.meta_data.create_href());
                    let title = post.meta_data.title.clone();
                    let date = post.meta_data.date.clone();
                    let tag_label = post.meta_data.tag_label();
                    let tag_class = post.meta_data.tag_class();

                    view! {
                        <li class="post-row">
                            <a class="post-row__link" href=href>
                                <span class="post-row__title">{title}</span>
                                <span class="post-row__meta">
                                    <span class="post-row__date">{date}</span>
                                    <span class=tag_class>{tag_label}</span>
                                </span>
                            </a>
                        </li>
                    }
                })
                .collect_view()
                .into_any()
        })
    };

    view! {
        <SeoMeta
            title="Blog"
            description="Notes on software, design, and travel from Phase Alpha."
            path="/blog"
        />
        <Layout buffer="*blog*" mode="(Org)" status=count>
            <section class="section">
                <span class="eyebrow">";; ~/phase-alpha/blog/index.org"</span>
                <h1>"Blog"</h1>
                <p class="hero__tagline">"Notes on software, design, and travel."</p>
                <Suspense fallback=move || {
                    view! { <p class="card__desc">"Loading posts…"</p> }
                }>
                    <ul class="post-list" style="margin-top: var(--sp-5)">{rows}</ul>
                </Suspense>
            </section>
        </Layout>
    }
}

#[component]
pub fn BlogTagArchive() -> impl IntoView {
    let params = use_params_map();
    let tag_param = move || params.with(|params| params.get("tag").unwrap_or_default().to_string());

    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find posts resource");

    let body = move || {
        let tag = tag_param();
        posts.and_then(move |posts| {
            let tag = tag.clone();
            let heading = format!(":{tag}:");
            let description = format!("Posts tagged {tag} on the Phase Alpha blog.");
            let path = format!("/blog/tag/{tag}");

            let filtered: Vec<_> = posts
                .iter()
                .filter(|p| p.meta_data.tags.contains(&tag))
                .collect();

            let rows = if filtered.is_empty() {
                view! { <p class="card__desc">"No posts with this tag yet."</p> }.into_any()
            } else {
                filtered
                    .iter()
                    .map(|post| {
                        let href = format!("/blog/{}", post.meta_data.create_href());
                        let title = post.meta_data.title.clone();
                        let date = post.meta_data.date.clone();

                        view! {
                            <li class="post-row">
                                <a class="post-row__link" href=href>
                                    <span class="post-row__title">{title}</span>
                                    <span class="post-row__meta">
                                        <span class="post-row__date">{date}</span>
                                    </span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()
                    .into_any()
            };

            view! {
                <SeoMeta
                    title=format!("Posts tagged {tag}")
                    description=description
                    path=path
                />
                <span class="eyebrow">";; ~/phase-alpha/blog/tag.org"</span>
                <h1>{heading}</h1>
                <ul class="post-list" style="margin-top: var(--sp-5)">{rows}</ul>
                <a class="more-link" href="/blog">"→ all posts"</a>
            }
            .into_any()
        })
    };

    view! {
        <Layout buffer="*blog-tag*" mode="(Org)">
            <section class="section">
                <Suspense fallback=move || {
                    view! { <p class="card__desc">"Loading posts…"</p> }
                }>{body}</Suspense>
            </section>
        </Layout>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let post_slug =
        move || params.with(|params| params.get("post").unwrap_or_default().to_string());

    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>()
        .expect("posts resource should be provided");

    let body = move || {
        let slug = post_slug();
        posts.and_then(move |posts| {
            if redirect_legacy_post(&slug, posts) {
                return view! { <p class="card__desc">"Redirecting…"</p> }.into_any();
            }

            match posts.iter().find(|p| p.meta_data.create_href() == slug) {
                Some(post) => {
                    let title = post.meta_data.title.clone();
                    let heading = post.meta_data.title.clone();
                    let description = post.meta_data.description.clone();
                    let date = post.meta_data.date.clone();
                    let content = post.content.clone();
                    let href = post.meta_data.create_href();
                    let path = format!("/blog/{href}");
                    let image_url = absolute_image(&post.meta_data.image_path);
                    let primary_tag = post.meta_data.tags.first().cloned();
                    let filetags = if post.meta_data.tags.is_empty() {
                        String::new()
                    } else {
                        format!("#+filetags: {}", post.meta_data.tag_label())
                    };

                    let json_ld = serde_json::json!({
                        "@context": "https://schema.org",
                        "@type": "BlogPosting",
                        "headline": title.clone(),
                        "description": description.clone(),
                        "image": image_url,
                        "datePublished": date.clone(),
                        "author": { "@type": "Organization", "name": "Phase Alpha" },
                        "publisher": {
                            "@type": "Organization",
                            "name": "Phase Alpha",
                            "logo": { "@type": "ImageObject", "url": absolute_url("/palogo.png") },
                        },
                        "mainEntityOfPage": { "@type": "WebPage", "@id": absolute_url(&path) },
                    })
                    .to_string();

                    let tag_link = primary_tag.clone().map(|tag| {
                        let tag_href = format!("/blog/tag/{tag}");
                        view! {
                            <a class="more-link" href=tag_href>{format!("→ more :{tag}: posts")}</a>
                        }
                    });

                    let related = primary_tag
                        .map(|tag| {
                            posts
                                .iter()
                                .filter(|p| {
                                    p.meta_data.create_href() != href
                                        && p.meta_data.tags.first() == Some(&tag)
                                })
                                .take(RELATED_COUNT)
                                .cloned()
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    let related_section = if related.is_empty() {
                        None
                    } else {
                        let items = related
                            .into_iter()
                            .map(|related_post| {
                                let related_href =
                                    format!("/blog/{}", related_post.meta_data.create_href());
                                let related_title = related_post.meta_data.title.clone();
                                view! {
                                    <li class="post-row">
                                        <a class="post-row__link" href=related_href>
                                            <span class="post-row__title">{related_title}</span>
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view();
                        Some(view! {
                            <section class="section">
                                <span class="eyebrow">";; related posts"</span>
                                <ul class="post-list">{items}</ul>
                            </section>
                        })
                    };

                    view! {
                        <SeoMeta
                            title=title.clone()
                            description=description.clone()
                            path=path
                            image=post.meta_data.image_path.clone()
                            og_type="article".to_string()
                        />
                        <Script type_="application/ld+json">{json_ld}</Script>

                        // Org keyword preamble, shown literally as in the
                        // mockup. Rendered from the front matter; the posts
                        // themselves remain Markdown.
                        <div class="org-keywords">
                            <div>{format!("#+title: {title}")}</div>
                            <div class="org-keywords__line">
                                <span>{format!("#+date: {date}")}</span>
                                <span>{filetags}</span>
                            </div>
                        </div>

                        <header class="post-header">
                            <h1>{heading}</h1>
                            <p class="post-header__subtitle">{description}</p>
                        </header>

                        // The `*` / `**` heading markers are drawn by CSS on
                        // `.prose h1/h2/h3`, so no org parsing is involved.
                        <div class="prose" inner_html=content></div>

                        {tag_link}
                        {related_section}
                    }
                    .into_any()
                }
                None => view! {
                    <h1>"Post not found"</h1>
                    <p class="hero__tagline">{format!("Nothing here matches \"{slug}\".")}</p>
                    <a class="more-link" href="/blog">"→ all posts"</a>
                }
                .into_any(),
            }
        })
    };

    view! {
        <Layout buffer="*blog*" mode="(Org)">
            <article class="section">
                <Suspense fallback=move || {
                    view! { <p class="card__desc">"Loading post…"</p> }
                }>{body}</Suspense>
            </article>
        </Layout>
    }
}
