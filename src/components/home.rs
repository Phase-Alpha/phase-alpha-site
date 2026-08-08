use crate::components::layout::Layout;
use crate::server_functions::posts::*;
use leptos::prelude::*;

/// How many posts the homepage teaser shows.
const TEASER_COUNT: usize = 3;

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find posts resource");

    // Text-only teaser. The previous design put large photo cards here, which
    // the notes identified as the biggest structural problem: the personal blog
    // was competing with the client-facing pitch for the same scroll.
    let teaser = move || {
        posts.and_then(|posts| {
            posts
                .iter()
                // `take` rather than slicing. The old `posts[0..=2]` panicked
                // if there were ever fewer than three posts.
                .take(TEASER_COUNT)
                .map(|post| {
                    let href = format!("/blog/{}", post.meta_data.create_href());
                    let title = post.meta_data.title.clone();
                    let tag_label = post.meta_data.tag_label();
                    let tag_class = post.meta_data.tag_class();

                    view! {
                        <li class="post-row">
                            <a class="post-row__link" href=href>
                                <span class="post-row__title">{title}</span>
                                <span class="post-row__meta">
                                    <span class=tag_class>{tag_label}</span>
                                </span>
                            </a>
                        </li>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <Layout buffer="*phase-alpha*" mode="(Fundamental)">
            <section class="section">
                <img
                    class="hero__logo"
                    src="/palogo.png"
                    alt="Phase Alpha logo"
                    width="56"
                    height="56"
                />
                <span class="eyebrow">";; ~/phase-alpha/README"</span>
                <h1>"Phase Alpha"</h1>
                <p class="hero__tagline">
                    "Custom software and design for small teams who need to ship something real."
                </p>
                <div class="btn-row">
                    <a class="btn btn--primary" href="/services">"See our work"</a>
                    <a class="btn" href="/contact">"Get in touch"</a>
                </div>
            </section>

            <section class="section">
                <h2>"What we do"</h2>
                <div class="grid-2" style="margin-top: var(--sp-5)">
                    <div>
                        <span class="eyebrow eyebrow--green">";; software"</span>
                        <h3 class="card__title">"Software & automation"</h3>
                        <p class="card__desc">
                            "Custom tools, workflow automation, and upgrades to legacy systems."
                        </p>
                    </div>
                    <div>
                        <span class="eyebrow eyebrow--magenta">";; design"</span>
                        <h3 class="card__title">"Design & writing"</h3>
                        <p class="card__desc">
                            "UI/UX design, engineering and graphic design, brand copy and editing."
                        </p>
                    </div>
                </div>
            </section>

            // Positioned after the pitch on purpose: proof of activity, not
            // the main event.
            <section class="section">
                <span class="eyebrow">";; recent notes"</span>
                <Suspense fallback=move || {
                    view! { <p class="card__desc">"Loading notes…"</p> }
                }>
                    <ul class="post-list">{teaser}</ul>
                </Suspense>
                <a class="more-link" href="/blog">"→ all posts"</a>
            </section>
        </Layout>
    }
}
