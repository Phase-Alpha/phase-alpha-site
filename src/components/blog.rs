// use crate::components::navigation::*;
// use leptos::*;
// use leptos_meta::*;

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

// #[component]
// pub fn Blog(cx: Scope) -> impl IntoView {
//     // load the posts
//     let posts = create_resource(|| (), |_| async { list_post_metadata().await });
//     let posts_view = move || {
//         posts.and_then(|posts| {
//                 posts.iter()
//                     .map(|post| view! {
//                         <li>
//                             <a href=format!("/blog/post/{}", post.id)>{&post.title}</a> "|"
//                             <a href=format!("/post_in_order/{}", post.id)>{&post.title}"(in order)"</a>
//                         </li>
//                     })
//                     .collect_view()
//             })
//     };

//     view! { cx,

//         <body>

//             <header id="header">
//                 <a href="" class="title">PhaseAlpha</a>
//                 <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
//             </header>

//             <div id="wrapper">

//                     <section id="main" class="wrapper">
//                         <div class="inner">
//                             <h1 class="major">Blog Posts</h1>
//                             <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
//                                 <ul>{posts_view}</ul>
//                             </Suspense>
//                         </div>
//                     </section>

//             </div>

//             // <footer id="footer" class="wrapper alt">
//             //     <div class="inner">
//             //     </div>
//             // </footer>

//     </body>
//     }
// }
