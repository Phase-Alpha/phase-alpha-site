use crate::components::navigation::*;
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Blog() -> impl IntoView {
    // Load the posts resource from context
    let posts = match use_context::<Resource<(), Result<Vec<Post>, ServerFnError>>>() {
        Some(resource) => resource,
        None => {
            // Create a new resource if not found in context
            let resource = Resource::new(
                || (),
                |_| async move { get_posts("posts/".to_string()).await },
            );
            provide_context(resource.clone());
            resource
        }
    };

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
                        {move || {
                            posts.read().map(|result| {
                                match result {
                                    Ok(posts_list) => {
                                        if posts_list.is_empty() {
                                            view! { <p>"No posts available"</p> }
                                        } else {
                                            view! {
                                                <div class="post-list-container">
                                                    <ul class="post-list">
                                                        {posts_list.iter()
                                                            .map(|post| view! {
                                                                <li>
                                                                    <a href=format!("/blog/{}", post.meta_data.clone().create_href())>
                                                                        {post.meta_data.title.clone()}
                                                                    </a>
                                                                </li>
                                                            })
                                                            .collect_view()}
                                                    </ul>
                                                </div>
                                            }
                                        }
                                    },
                                    Err(_) => view! { <p>"Error loading posts"</p> }
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </section>
        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    // Load the posts resource from context
    let posts = match use_context::<Resource<(), Result<Vec<Post>, ServerFnError>>>() {
        Some(resource) => resource,
        None => {
            // Create a new resource if not found in context
            let resource = Resource::new(
                || (),
                |_| async move { get_posts("posts/".to_string()).await },
            );
            provide_context(resource.clone());
            resource
        }
    };
    
    // Get the post slug from URL parameters
    let params = use_params_map();
    let query = move || params.get().get("post").unwrap_or_default().to_string();
    
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
                            posts.read().map(|result| {
                                match result {
                                    Ok(all_posts) => {
                                        let found_post = all_posts.iter()
                                            .find(|p| p.meta_data.clone().create_href() == query());
                                        
                                        match found_post {
                                            Some(post) => view! {
                                                <div class="post-content">
                                                    <h1 class="major">{post.meta_data.title.clone()}</h1>
                                                    <div inner_html=post.clone().content/>
                                                </div>
                                            },
                                            None => view! { 
                                                <div class="post-content">
                                                    <h1 class="major">"Post not found"</h1>
                                                </div>
                                            }
                                        }
                                    },
                                    Err(_) => view! { <p>"Error loading post"</p> }
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </section>
        </div>
    }
}