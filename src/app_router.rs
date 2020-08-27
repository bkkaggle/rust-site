use yew::prelude::*;
use yew_router::{prelude::*, Switch};

use crate::components::footer::Footer;
use crate::routes::{blog::Blog, index::Index};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Index,
}

pub struct AppRouter {}

impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="flex-horizontal wrap">
                    <div class="flex-vertical" id="sidebar">
                        <div class="flex-horizontal">
                            <RouterButton<AppRoute> route=AppRoute::Index classes="block"> {"Home"} </RouterButton<AppRoute>>
                            <RouterButton<AppRoute> route=AppRoute::Blog classes="block"> {"Blog"} </RouterButton<AppRoute>>
                        </div>
                        <img class="profile-pic" src="https://avatars1.githubusercontent.com/u/29356759?u=82124ceed1df5a85de458689cd5ee572b34d8ee9&v=4" alt="Goose profile picture" />
                        <div class="block">
                            <h1>{"I'm Bilal"}</h1>
                            <h3>{"UWaterloo SE '25 — I like working on ML and systems programming projects"}</h3>
                            <h4>
                            {"In the past, you could find me "} <a href="/explore">{"training neural networks"}</a> {", writing "} <a href="https://github.com/bkkaggle/L2">{"fast machine learning libraries"}</a> {", and doing full-stack and mobile development."}
                            </h4>
                        </div>

                        <div class="block" id="social-links">
                            <div class="flex-horizontal">
                                <a href="https://kaggle.com/bkkaggle">
                                    <img
                                        alt="Kaggle profile for Bilal Khan"
                                        src="https://kaggle.com/static/images/favicon.ico"
                                        height="20"
                                    />
                                </a>

                                <a href="https://medium.com/@bkkaggle">
                                    <img
                                        alt="Medium profile for Bilal Khan"
                                        src="https://miro.medium.com/max/195/1*emiGsBgJu2KHWyjluhKXQw.png"
                                        height="20"
                                    />
                                </a>

                                <a href="https://www.linkedin.com/in/bilal-khan-9294451ab/">
                                    <img
                                        alt="LinkedIn profile for Bilal Khan"
                                        src="https://content.linkedin.com/content/dam/me/business/en-us/amp/brand-site/v2/bg/LI-Bug.svg.original.svg"
                                        height="20"
                                    />
                                </a>

                                <a href="https://dev.to/bkkaggle">
                                    <img
                                        alt="Dev.to profile for Bilal Khan"
                                        src="https://d2fltix0v2e0sb.cloudfront.net/dev-badge.svg"
                                        height="20"
                                    />
                                </a>

                                <a href="mailto:bilal.khan1@uwaterloo.ca">
                                    <img
                                        alt="mail to bilal.khan1@uwaterloo.ca"
                                        src="https://prosmartmailing.com/wp-content/uploads/2016/09/Mail-Circle-Icon.png"
                                        height="20"
                                    />
                                </a>

                            </div>

                            <div class="flex-horizontal" id="discord">
                                <img
                                    alt="Discord icon"
                                    src="https://discord.com/assets/f8389ca1a741a115313bede9ac02e2c0.svg"
                                    height="24"
                                />
                                <span>
                                    {"bkkaggle#8916"}
                                </span>
                            </div>
                        </div>

                    </div>

                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Index => html!{<Index />},
                                AppRoute::Blog => html! {<Blog />},
                            }
                        })
                    />
                </div>
                <Footer />
            </>
        }
    }
}
