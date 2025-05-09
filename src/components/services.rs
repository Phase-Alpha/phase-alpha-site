use crate::components::navigation::*;
use leptos::prelude::*;
#[component]
pub fn Services() -> impl IntoView {
    view! {

        <header id="header">
            <a href="" class="title">PhaseAlpha</a>
            <Nav exclude={Some(NavElements::None)} current_page={NavElements::Services}/>
        </header>

        <div id="wrapper">

                <section id="main" class="wrapper">
                    <div class="inner">
                        <h1 class="major">Services</h1>
                        <span class="image fit"><img src="services.png" alt="" /></span>
                        <p>At Phase Alpha, our specialties and interests transcend industry boundaries.</p>
                        <p>With our main branches; technical, and creative,
                        we can help you realise your vision.</p>
                        <p>Some of our previous work has involved:</p>
                        <ol>
                            <li>Software;
                                <ul>
                                    <li>Process and Design Automation</li>
                                    <li>Bespoke Applications</li>
                                    <li>Maintenance and upgrading of legacy systems</li>
                                </ul>
                            </li>
                            <li>Design;
                                <ul>
                                    <li>Engineering</li>
                                    <li>Graphic/Illustration</li>
                                </ul>
                            </li>
                            <li>Writing;
                                <ul>
                                    <li>Creative</li>
                                    <li>Copy</li>
                                    <li>Proof</li>
                                </ul>
                            </li>
                        </ol>
                    </div>
                </section>

        </div>

    }
}
