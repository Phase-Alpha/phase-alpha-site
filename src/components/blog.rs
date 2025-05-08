use crate::components::navigation::*;
use crate::server_functions::posts::*;
use leptos::*;
use leptos_router::{use_params_map, A};

#[component]
pub fn Blog() -> impl IntoView {
    // load the posts
    let posts = use_context::<Resource<(), Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find context");
    let posts_view = move || {
        posts.and_then(|posts| {
                posts.iter()
                    .map(|post| view! {
                        <li>
                            <a href=format!("/blog/{}", post.meta_data.clone().create_href())>{&post.meta_data.title}</a>
                        </li>
                    })
                    .collect_view()
            })
    };

    view! {

        <header id="header">
            <A href="" class="title">PhaseAlpha</A>
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
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let posts = use_context::<Resource<(), Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find context");
    let params = use_params_map();
    let query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());

    // load the posts
    let post_view = move || {
        posts.and_then(|posts| {
            posts
                .iter()
                .find(|&p| p.meta_data.clone().create_href() == query())
                .map(|post| {
                    view! {
                        <h1 class="major">{&post.meta_data.title}</h1>
                        <div inner_html=post.clone().content/>
                    }
                })
        })
    };
    view! {


        <header id="header">
            <A href="" class="title">PhaseAlpha</A>
            <Nav exclude={Some(NavElements::None)} current_page={NavElements::Blog}/>
        </header>

        <div id="wrapper">

                <section id="main" class="wrapper">
                    <div class="inner">
                        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                            {post_view}
                        </Suspense>
                    </div>
                </section>

        </div>


    }
}
