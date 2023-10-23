use crate::components::navigation::*;
use crate::server_functions::posts::{get_post, get_posts};
use crate::server_functions::*;
use leptos::*;
use leptos_router::{use_params, use_params_map};

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Post {
//     id: usize,
//     title: String,
//     content: String,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct PostMetadata {
//     id: usize,
//     title: String,
// }

// #[server]
// pub async fn list_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {
//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     Ok(POSTS
//         .iter()
//         .map(|data| PostMetadata {
//             id: data.id,
//             title: data.title.clone(),
//         })
//         .collect())
// }

// #[server]
// pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     Ok(POSTS.iter().find(|post| post.id == id).cloned())
// }

#[component]
pub fn Blog() -> impl IntoView {
    // load the posts
    let posts = create_resource(|| (), |_| async { get_posts("./posts/".to_string()).await });
    let posts_view = move || {
        posts.and_then(|posts| {
                posts.iter()
                    .map(|post| view! {
                        <li>
                            <a href=format!("/blog/post/{}", post.meta_data.clone().create_href())>{&post.meta_data.title}</a>
                        </li>
                    })
                    .collect_view()
            })
    };

    view! {
        <body>

            <header id="header">
                <a href="" class="title">PhaseAlpha</a>
                <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
            </header>

            <div id="wrapper">

                    <section id="main" class="wrapper">
                        <div class="inner">
                            <h1 class="major">Blog Posts</h1>
                            <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                                <ul>{posts_view}</ul>
                            </Suspense>
                        </div>
                    </section>

            </div>

            // <footer id="footer" class="wrapper alt">
            //     <div class="inner">
            //     </div>
            // </footer>

    </body>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let data = create_resource(
        move || params.with(|p| p.get("post").cloned().unwrap_or_default()),
        move |post| get_post(post, "./posts/".to_string()),
    );
    // load the posts
    // let post_view = move || {
    //     posts.and_then(|posts| {
    //         let post = get_post(href, *posts);
    //         return view! {
    //             post.content
    //         };
    //     })
    // };
    view! {
            <body>

                <header id="header">
                    <a href="" class="title">PhaseAlpha</a>
                    <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
                </header>

                <div id="wrapper">

                        <section id="main" class="wrapper">
                            <div class="inner">
                                <h1 class="major">{data.meta_data.clone().title}</h1>
                                <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                                    <ul>{data.content}</ul>
                                </Suspense>
                            </div>
                        </section>

                </div>

                // <footer id="footer" class="wrapper alt">
                //     <div class="inner">
                //     </div>
                // </footer>

        </body>
    }
}
