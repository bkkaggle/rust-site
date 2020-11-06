use yew::prelude::*;

pub struct Projects {}

impl Component for Projects {
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
            <div id="projects">
                <h1>{"My Open-Source Projects"}</h1>
                <div class="flex-horizontal wrap">
                    <div class="block project">
                        <div class="flex-horizontal">
                            <h2><a href="https://github.com/bilal2vec/l2">{"L2"}</a></h2>
                            <span>{"110+ ⭐️"}</span>
                        </div>
                        <h4>{"l2 is a fast, Pytorch-style Tensor+Autograd library written in Rust"}</h4>
                    </div>
                    <div class="block project">
                        <div class="flex-horizontal">
                            <h2><a href="https://github.com/bilal2vec/pytorch_zoo">{"PyTorch Zoo"}</a></h2>
                            <span>{"70+ ⭐️"}</span>
                        </div>
                        <h4>{"A collection of useful modules and utilities (especially helpful for kaggling) not available in Pytorch"}</h4>
                    </div>
                    <div class="block project">
                        <div class="flex-horizontal">
                            <h2><a href="https://github.com/bilal2vec/machine-learning-flight-rules">{"ML Flight Rules"}</a></h2>
                            <span>{"60 ⭐️"}</span>
                        </div>
                        <h4>{"A guide for astronauts (now, people doing machine learning) about what to do when things go wrong."}</h4>
                    </div>
                </div>
            </div>
        }
    }
}
