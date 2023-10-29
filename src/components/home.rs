use crate::components::navigation::*;
use crate::server_functions::posts::*;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let posts = use_context::<Resource<(), Result<Vec<Post>, ServerFnError>>>()
        .expect("unable to find context");

    let posts_view = move || {
        posts.and_then(|posts| {
                posts[0..=2].iter()
                    .map(|post| view! {
                        <section>
                            <a href=format!("/blog/post/{}", post.meta_data.clone().create_href()) class="image"><img src={&post.meta_data.image_path} alt="" data-position="center center" /></a>
                            <div class="content">
                                <div class="inner">
                                    <h2>{&post.meta_data.title}</h2>
                                    <p>{&post.meta_data.description}</p>
                                    <ul class="actions">
                                        <li><a href=format!("/blog/post/{}", post.meta_data.clone().create_href()) class="button">Read</a></li>
                                    </ul>
                                </div>
                            </div>
                        </section>
                    })
                    .collect_view()
            })
    };

    view! {
        <body>
            <section id="sidebar">
                <div class="inner">
                    <Nav exclude={Some(NavElements::None)} current_page={NavElements::Home}/>
                </div>
            </section>

            <div id="wrapper">

                    <section id="intro" class="wrapper style1 fullscreen fade-up">
                        <div class="inner">
                            <div class="inner">
                                <a class="image"><img src="palogo.png" alt="" data-position="center center" style="width: 25%; height: 25%; margin: auto"/></a>
                            </div>
                            <h1>Phase Alpha</h1>
                            <p>Welcome to our Atelier, your one stop shop for technical and creative pursuits</p>
                            <ul class="actions">
                                <li><a href="#two" class="button scrolly">Learn more</a></li>
                            </ul>
                        </div>
                    </section>

                    <section id="one" class="wrapper style2 spotlights">
                        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                            {posts_view}
                        </Suspense>
                    </section>

                    <section id="two" class="wrapper style3 fade-up">
                        <div class="inner">
                            <h2>What we do</h2>
                            <p>At PhaseAlpha, our specialties and interests transcend industry boundaries.

                            With our main branches; technical, and creative, we can help you realise your vision.</p>
                            <div class="features">
                                <section>
                                    <span class="icon solid major fa-code"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M392.8 1.2c-17-4.9-34.7 5-39.6 22l-128 448c-4.9 17 5 34.7 22 39.6s34.7-5 39.6-22l128-448c4.9-17-5-34.7-22-39.6zm80.6 120.1c-12.5 12.5-12.5 32.8 0 45.3L562.7 256l-89.4 89.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l112-112c12.5-12.5 12.5-32.8 0-45.3l-112-112c-12.5-12.5-32.8-12.5-45.3 0zm-306.7 0c-12.5-12.5-32.8-12.5-45.3 0l-112 112c-12.5 12.5-12.5 32.8 0 45.3l112 112c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256l89.4-89.4c12.5-12.5 12.5-32.8 0-45.3z" style="fill: #a04f8e;"/></svg></span>
                                    <h3>Software && Automation</h3>
                                    <p>From writing bespoke software to automating your current processes, we have experience in a variety of industries.</p>
                                </section>
                                <section>
                                    <span class="icon solid major"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M339.3 367.1c27.3-3.9 51.9-19.4 67.2-42.9L568.2 74.1c12.6-19.5 9.4-45.3-7.6-61.2S517.7-4.4 499.1 9.6L262.4 187.2c-24 18-38.2 46.1-38.4 76.1L339.3 367.1zm-19.6 25.4l-116-104.4C143.9 290.3 96 339.6 96 400c0 3.9 .2 7.8 .6 11.6C98.4 429.1 86.4 448 68.8 448H64c-17.7 0-32 14.3-32 32s14.3 32 32 32H208c61.9 0 112-50.1 112-112c0-2.5-.1-5-.2-7.5z" style="fill: #a04f8e;" /></svg></span>
                                    <h3>Design && Writing</h3>
                                    <p>Perhaps you need some UI/UX design, or proof reading and copywriting. They might seem mutually exclusive, but an element of creativity is necessary in most fields.</p>
                                </section>
                                // <section>
                                //     <span class="icon solid major fa-cog"></span>
                                //     <h3>Sed erat ullam corper</h3>
                                //     <p>Phasellus convallis elit id ullam corper amet et pulvinar. Duis aliquam turpis mauris, sed ultricies erat dapibus.</p>
                                // </section>
                                // <section>
                                //     <span class="icon solid major fa-desktop"></span>
                                //     <h3>Veroeros quis lorem</h3>
                                //     <p>Phasellus convallis elit id ullam corper amet et pulvinar. Duis aliquam turpis mauris, sed ultricies erat dapibus.</p>
                                // </section>
                                // <section>
                                //     <span class="icon solid major fa-link"></span>
                                //     <h3>Urna quis bibendum</h3>
                                //     <p>Phasellus convallis elit id ullam corper amet et pulvinar. Duis aliquam turpis mauris, sed ultricies erat dapibus.</p>
                                // </section>
                                // <section>
                                //     <span class="icon major fa-gem"></span>
                                //     <h3>Aliquam urna dapibus</h3>
                                //     <p>Phasellus convallis elit id ullam corper amet et pulvinar. Duis aliquam turpis mauris, sed ultricies erat dapibus.</p>
                                // </section>
                            </div>
                            <ul class="actions">
                                <li><a href="/services" class="button">Learn more</a></li>
                            </ul>
                        </div>
                    </section>

                    <section id="three" class="wrapper style1 fade-up">
                        <div class="inner">
                            <h2>Get in touch</h2>
                            <p>Have a project in mind? Contact us and lets chat!</p>
                            <div class="split style1">
                                <section>
                                    <form method="post" action="#">
                                        <div class="fields">
                                            <div class="field half">
                                                <label for="name">Name</label>
                                                <input type="text" name="name" id="name" />
                                            </div>
                                            <div class="field half">
                                                <label for="email">Email</label>
                                                <input type="text" name="email" id="email" />
                                            </div>
                                            <div class="field">
                                                <label for="message">Message</label>
                                                <textarea name="message" id="message" rows="5"></textarea>
                                            </div>
                                        </div>
                                        <ul class="actions">
                                            <li><a href="" class="button submit">Send Message</a></li>
                                        </ul>
                                    </form>
                                </section>
                                <section>
                                    <ul class="contact">
                                        <li>
                                            <h3>Address</h3>
                                            <span> The Worldwide Web </span>
                                        </li>
                                        <li>
                                            <h3>Email</h3>
                                            <a href="#">info@phasealpha.io</a>
                                        </li>
                                        // <li>
                                        //     <h3>Phone</h3>
                                        //     <span>(000) 000-0000</span>
                                        // </li>
                                        // <li>
                                        //     <h3>Social</h3>
                                        //     <ul class="icons">
                                        //         <li><a href="#" class="icon brands fa-twitter"><span class="label">Twitter</span></a></li>
                                        //         <li><a href="#" class="icon brands fa-facebook-f"><span class="label">Facebook</span></a></li>
                                        //         <li><a href="#" class="icon brands fa-github"><span class="label">GitHub</span></a></li>
                                        //         <li><a href="#" class="icon brands fa-instagram"><span class="label">Instagram</span></a></li>
                                        //         <li><a href="#" class="icon brands fa-linkedin-in"><span class="label">LinkedIn</span></a></li>
                                        //     </ul>
                                        // </li>
                                    </ul>
                                </section>
                            </div>
                        </div>
                    </section>

            </div>

            // <footer id="footer" class="wrapper alt">
            //         <div class="inner">
            //         </div>
            // </footer>

        </body>
    }
}
