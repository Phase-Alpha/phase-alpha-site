use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <Layout buffer="*services*" mode="(Org)">
            <section class="section">
                <span class="eyebrow">";; ~/phase-alpha/services.org"</span>
                <h1>"Services"</h1>
                <p class="hero__tagline">
                    "Our specialties transcend industry boundaries. With two main branches, "
                    "technical and creative, we can help you realise your vision."
                </p>
            </section>

            <section class="section">
                <div class="grid-2">
                    <div>
                        <span class="eyebrow eyebrow--green">";; software"</span>
                        <h3 class="card__title">"Software & automation"</h3>
                        <ul class="prose card__desc">
                            <li>"Process and design automation"</li>
                            <li>"Bespoke applications"</li>
                            <li>"Maintenance and upgrading of legacy systems"</li>
                        </ul>
                    </div>
                    <div>
                        <span class="eyebrow eyebrow--magenta">";; design"</span>
                        <h3 class="card__title">"Design"</h3>
                        <ul class="prose card__desc">
                            <li>"Engineering"</li>
                            <li>"Graphic and illustration"</li>
                            <li>"UI/UX"</li>
                        </ul>
                    </div>
                    <div>
                        <span class="eyebrow">";; writing"</span>
                        <h3 class="card__title">"Writing"</h3>
                        <ul class="prose card__desc">
                            <li>"Creative"</li>
                            <li>"Copy"</li>
                            <li>"Proofreading"</li>
                        </ul>
                    </div>
                </div>
            </section>

            <section class="section">
                <span class="eyebrow">";; next"</span>
                <h2>"Start a conversation"</h2>
                <p class="card__desc" style="margin-top: var(--sp-2)">
                    "Tell us what you are trying to build and we will tell you how we would "
                    "approach it."
                </p>
                <div class="btn-row">
                    <a class="btn btn--primary" href="/contact">"Get in touch"</a>
                </div>
            </section>
        </Layout>
    }
}
