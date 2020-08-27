use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
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
            <div class="flex-horizontal space-between wrap" id="footer">
                <div>
                    {"© Bilal Khan, 2020"}
                </div>

                <div>
                    {"made with Rust, "}<a href="https://yew.rs/">{"Yew"}</a>{", and "}<a href="https://thesephist.github.io/blocks.css/">{"blocks.css"}</a>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
