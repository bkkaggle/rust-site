use yew::prelude::*;

use crate::components::{projects::Projects, resume::Resume};

pub struct Index {}

impl Component for Index {
    type Properties = ();
    type Message = ();

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
            <div id="right">
                // <Projects />
                // <Resume />
            </div>
        }
    }
}
