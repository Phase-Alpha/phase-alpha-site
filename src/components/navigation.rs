use leptos::*;
use leptos_router::A;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavElements {
    Blog,
    Home,
    About,
    Contact,
    Services,
    None,
}

#[component]
pub fn Nav(exclude: Option<NavElements>, current_page: NavElements) -> impl IntoView {
    let exclude = exclude.unwrap_or(NavElements::None);
    let nav_elements_index = vec![
        (NavElements::Home, "#intro", "WELCOME"),
        (NavElements::Blog, "/blog", "BLOG"),
        (NavElements::About, "#two", "WHAT WE DO"),
        (NavElements::Contact, "#three", "GET IN TOUCH"),
    ];

    let nav_elements_other = vec![
        (NavElements::Home, "/", "HOME"),
        (NavElements::Blog, "/blog", "BLOG"),
    ];

    let nav_elements;
    match current_page {
        NavElements::Home => {
            nav_elements = nav_elements_index
                .into_iter()
                .filter(|(e, _, _)| *e != exclude)
                .map(|(e, href, text)| {
                    let exact = e == NavElements::Home;
                    view! {
                        <li><A exact=exact href=href><p>{text}</p></A></li>
                    }
                })
                .collect::<Vec<_>>();
        }
        _ => {
            nav_elements = nav_elements_other
                .into_iter()
                .filter(|(e, _, _)| *e != exclude)
                .map(|(e, href, text)| {
                    let exact = e == NavElements::Home;
                    view! {
                        <li><A exact=exact href=href><p>{text}</p></A></li>
                    }
                })
                .collect::<Vec<_>>();
        }
    }

    view! {
        <nav>
            <ul>
                {nav_elements}
            </ul>
        </nav>
    }
}
