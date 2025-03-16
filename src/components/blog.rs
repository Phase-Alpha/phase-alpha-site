use crate::components::navigation::*;
use crate::server_functions::posts::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_params_map;

#[component]
pub fn Blog() -> impl IntoView {
    // load the posts
    let posts = use_context::<Result<Vec<Post>, ServerFnError>>().expect("unable to find context");
    let posts_view = view! {
        {move || {
            let posts = posts.clone();
            posts.map(|posts| {
                posts
                .iter()
                .map(|post| view! {
                    <li>
                        <a href=format!("/blog/{}", post.meta_data.clone().create_href())>{post.meta_data.title.clone()}</a>
                    </li>
                }).collect_view()
            })
        }}
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
                            <ul>{posts_view}</ul>
                        </Suspense>
                    </div>
                </section>

        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let posts = use_context::<Result<Vec<Post>, ServerFnError>>().expect("unable to find context");
    let params = use_params_map();
    let query = move || params.with(|params| params.get("post").unwrap_or_default());

    // load the posts
    let post_view = view! {
        move || {
            posts.map(|posts| {
                posts
                    .iter()
                    .find(|p| p.meta_data.clone().create_href() == query())
                    .map(|post| {
                        view! {
                            <h1 class="major">{post.meta_data.title.clone()}</h1>
                            <div inner_html=post.clone().content/>
                        }
                    })
                })
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
                        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                            {post_view}
                        </Suspense>
                    </div>
                </section>

        </div>


    }
}
