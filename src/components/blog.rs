use crate::components::navigation::*;
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Blog() -> impl IntoView {
    // Get posts resource from context
    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>().expect("unable to find posts resource");
    
    view! {
        <header id="header">
            <a href="" class="title">PhaseAlpha</a>
            <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
        </header>

        <div id="wrapper">
            <section id="main" class="wrapper">
                <div class="inner">
                    <h1 class="major">Blog Posts</h1>
                    <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                        <ul>
                            {move || {
                                match posts.get() {
                                    None => view! { <p>"Loading posts..."</p> }.into_any(),
                                    Some(Err(_)) => view! { <p>"Error loading posts"</p> }.into_any(),
                                    Some(Ok(posts_list)) => {
                                        if posts_list.is_empty() {
                                            view! { <p>"No posts available"</p> }.into_any()
                                        } else {
                                            view! {
                                                <ul class="post-list">
                                                    {posts_list.iter().map(|post| {
                                                        let href = post.meta_data.create_href();
                                                        view! {
                                                            <li>
                                                                <a href=format!("/blog/{}", href)>
                                                                    {post.meta_data.title.clone()}
                                                                </a>
                                                            </li>
                                                        }
                                                    }).collect_view()}
                                                </ul>
                                            }.into_any()
                                        }
                                    }
                                }
                            }}
                        </ul>
                    </Suspense>
                </div>
            </section>
        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    // Get posts resource from context
    let posts = use_context::<Resource<Result<Vec<Post>, ServerFnError>>>().expect("unable to find posts resource");
    
    // Get route parameters
    let params = use_params_map();
    let post_slug = move || params.with(|params| params.get("post").unwrap_or_default().to_string());

    view! {
        <header id="header">
            <a href="" class="title">PhaseAlpha</a>
            <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
        </header>

        <div id="wrapper">
            <section id="main" class="wrapper">
                <div class="inner">
                    <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
                        {move || {
                            let slug = post_slug();
                            match posts.get() {
                                None => view! { <p>"Loading post..."</p> }.into_any(),
                                Some(Err(_)) => view! { <p>"Error loading post"</p> }.into_any(),
                                Some(Ok(all_posts)) => {
                                    let found_post = all_posts.iter()
                                        .find(|p| p.meta_data.create_href() == slug);
                                    
                                    match found_post {
                                        Some(post) => view! {
                                            <h1 class="major">{post.meta_data.title.clone()}</h1>
                                            <div inner_html=post.content.clone()/>
                                        }.into_any(),
                                        None => view! {
                                            <h1 class="major">"Post not found"</h1>
                                            <p>"Looking for: "{slug}</p>
                                            <p><a href="/blog">"Back to blog list"</a></p>
                                        }.into_any()
                                    }
                                }
                            }
                        }}
                    </Suspense>
                </div>
            </section>
        </div>
    }
}
