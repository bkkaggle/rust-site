use yew::prelude::*;
use yew_router::{prelude::*, Switch};

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
            <div>
                <div>
                    <RouterButton<AppRoute> route=AppRoute::Index classes="block"> {"Home"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::Blog classes="block"> {"Blog"} </RouterButton<AppRoute>>
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
        }
    }
}
