use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// Tab-bar entries: href, visible label, and whether the link leaves the site.
///
/// Modelled on the Emacs tab-bar rather than a which-key popup, so navigation
/// is always visible instead of hidden behind a trigger.
const TABS: [(&str, &str, bool); 5] = [
    ("/", "home", false),
    ("/blog", "blog", false),
    ("/services", "services", false),
    ("/contact", "contact", false),
    ("https://phaseatelier.etsy.com", "shop", true),
];

/// Workspace chrome shared by every page: tab-bar, mode line, content, status
/// bar and echo area.
///
/// The redesign notes call for this to be the one consistent piece of literal
/// Emacs furniture, which is what makes the site read as a single workspace
/// rather than a themed skin. Individual sections are deliberately not wrapped
/// in fake window decoration.
#[component]
pub fn Layout(
    /// Mode line buffer name, e.g. `*phase-alpha*`.
    #[prop(into)]
    buffer: String,
    /// Major mode, shown in the mode line and status bar, e.g. `(Org)`.
    #[prop(into)]
    mode: String,
    /// Optional trailing status bar segment, e.g. `16 entries`.
    ///
    /// A signal rather than a plain string because callers may derive it from
    /// async data, as the blog index does with its post count.
    #[prop(optional)]
    status: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    // `pathname` is a Copy signal, so each tab's closure can capture it.
    let pathname = use_location().pathname;

    let tabs = TABS
        .iter()
        .map(|(href, label, external)| {
            let (href, label, external) = (*href, *label, *external);

            let is_current = move || {
                if external {
                    return false;
                }
                let path = pathname.get();
                if href == "/" {
                    path == "/"
                } else {
                    // Keep the tab marked while on a child route, so a blog
                    // post still highlights `blog`.
                    path == href || path.starts_with(&format!("{href}/"))
                }
            };

            view! {
                <li class="tabbar__item">
                    <a
                        class="tabbar__link"
                        class:tabbar__external=external
                        href=href
                        target=if external { Some("_blank") } else { None }
                        rel=if external { Some("noopener noreferrer") } else { None }
                        aria-current=move || if is_current() { Some("page") } else { None }
                    >
                        {label}
                    </a>
                </li>
            }
        })
        .collect_view();

    let mode_for_status = mode.clone();

    // `Option<Signal<_>>` is Copy, so this closure can capture it directly.
    let status_text = move || {
        status
            .map(|s| s.get())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| String::from("UTF-8"))
    };

    view! {
        // Plain anchors rather than the router's `A`. If the router intercepts
        // them we get client-side navigation; if it does not, we get a normal
        // page load, which is perfectly fine for a mostly static site.
        <nav class="tabbar" aria-label="Main">
            <ul class="tabbar__list">{tabs}</ul>
        </nav>

        <div class="modeline">
            <span class="modeline__buffer">"-:--- " {buffer}</span>
            <span class="modeline__meta">
                <span>{mode}</span>
                <span>"All"</span>
                <span>"L1"</span>
            </span>
        </div>

        <main class="page">{children()}</main>

        <div class="statusbar">
            // The active theme name follows prefers-color-scheme. Both labels
            // are rendered and CSS reveals the right one, since the server
            // cannot know which scheme the visitor prefers.
            <span>
                <span class="only-light">"modus-operandi"</span>
                <span class="only-dark">"modus-vivendi"</span>
            </span>
            <span class="statusbar__segments">
                <span>{mode_for_status}</span>
                <span>"Git:main"</span>
                <span>{status_text}</span>
            </span>
        </div>

        <div class="echo">
            <span class="echo__key">";; "</span>
            "Phase Alpha — phasealpha.io"
        </div>
    }
}
