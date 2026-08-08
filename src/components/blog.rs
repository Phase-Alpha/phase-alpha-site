use crate::components::layout::Layout;
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

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
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let post_slug =
        move || params.with(|params| params.get("post").unwrap_or_default().to_string());

    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>()
        .expect("posts resource should be provided");

    let body = move || {
        let slug = post_slug();
        posts.and_then(move |posts| {
            match posts.iter().find(|p| p.meta_data.create_href() == slug) {
                Some(post) => {
                    let title = post.meta_data.title.clone();
                    let heading = post.meta_data.title.clone();
                    let description = post.meta_data.description.clone();
                    let date = post.meta_data.date.clone();
                    let content = post.content.clone();
                    let filetags = if post.meta_data.tags.is_empty() {
                        String::new()
                    } else {
                        format!("#+filetags: {}", post.meta_data.tag_label())
                    };

                    view! {
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
