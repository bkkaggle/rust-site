use yew::prelude::*;
use yew_router::{prelude::*, Switch};

use crate::components::footer::Footer;
use crate::routes::{index::Index, uses::Uses};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/uses"]
    Uses,
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
                            <RouterButton<AppRoute> route=AppRoute::Uses classes="block"> {"Uses"} </RouterButton<AppRoute>>

                            <a href="https://drive.google.com/file/d/1cPg7WyNQ4hIzEw58KPy5SSEXsa2rd1LD/view?usp=sharing">
                                <div class="block"><span>{"Resume"}</span></div>
                            </a>

                            <a href="https://bkkaggle.github.io/blog">
                                <div class="block accent"><span>{"Blog"}</span></div>
                            </a>
                        </div>
                        <img class="profile-pic" src="https://avatars1.githubusercontent.com/u/29356759?u=82124ceed1df5a85de458689cd5ee572b34d8ee9&v=4" alt="Goose profile picture" />
                        <div class="block">
                            <h1>{"I'm Bilal"}</h1>
                            <h3>{"UWaterloo SE '25 — I like working on ML and systems programming projects"}</h3>
                            <h4>
                            {"In the past, you could find me competing on Kaggle, training neural networks, writing "} <a href="https://github.com/bkkaggle/L2">{"fast machine learning libraries"}</a> {", and doing full-stack and mobile development."}
                            </h4>
                        </div>

                        <div class="block" id="social-links">
                            <div class="flex-horizontal">

                                <a href="https://github.com/bkkaggle">
                                    <span style="color:#24292e"><svg aria-hidden="false" aria-label="GitHub" class="mr-2" height="20" role="img" viewBox="0 0 16 16" width="20" style="display:inline-block;fill:currentColor;user-select:none;vertical-align:middle"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"></path></svg></span>
                                </a>

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
                                AppRoute::Uses => html! {<Uses />},
                            }
                        })
                    />
                </div>
                <Footer />
            </>
        }
    }
}
