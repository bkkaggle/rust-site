use yew::prelude::*;
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
                <div class="flex-horizontal center">
                    <div class="flex-vertical stack">
                        <div class="flex-horizontal left" id="buttons">
                            <a href="/">
                                <div class="block"><span>{"home"}</span></div>
                            </a>
                            <a href="https://drive.google.com/file/d/1cPg7WyNQ4hIzEw58KPy5SSEXsa2rd1LD/view?usp=sharing">
                                <div class="block"><span>{"resume"}</span></div>
                            </a>
                            <a href="https://bkkaggle.github.io/blog">
                                <div class="block accent"><span>{"blog"}</span></div>
                            </a>
                        </div>
                        // <img class="profile-pic" src="https://avatars1.githubusercontent.com/u/29356759?u=82124ceed1df5a85de458689cd5ee572b34d8ee9&v=4" alt="Goose profile picture" />
                        <div class="left">
                            <h1>{"hi i'm bilal!"}</h1>
                        </div>
                        <h4>{"uwaterloo software engineering '25 — i'm currently interested in machine learning research, particularly in generative modeling."}</h4>
                        // <h4>{"things i've done (or failed to do):"}</h4>
                        <ul>
                            <li>{"i used to have an unhealthy obsession with spending way too much time competing in "} <a href="https://www.kaggle.com/bkkaggle">{"kaggle"}</a>{" competitions for a couple of years."}</li>
                            <li>{"i did a lot of full-stack dev work, mostly with react/react native and node, but i'm pretty out of touch with whatever js frameworks are cool nowadays."}</li>
                            <li>{"i"} <a href="https://github.com/bkkaggle/L2">{" wrote "}</a> {"a machine learning library in rust, which was a really fun language to work with—it's hard to beat the feeling of rewriting a few thousand lines of code and having them compile+pass all the test cases on the first try."}</li>
                            <li>{"i started going for a 5k run every morning during quarantine (keeping this up during school is still a wip)."}</li>
                            <li>{"this website runs on rust+webassembly."}</li>
                            <li>{"i once wrote what was essentially a very sus but also impressively feature-rich server-side rendering web framework in plain php by concatenating together *very* long strings of html based on data pulled from a db."}</li>
                            <li>{"^ was waay back before 12yo me even knew that things like github, react, or js frameworks existed, i was pretty clueless back then."}</li>
                            <li>{"i can speak 4 languages (english, urdu, french, and japanese) in diminishing levels of proficiency."}</li>
                            <li>{"i've visited several countries on 4 continents so far (covid has put a freeze on those plans for now :blobsad:)."}</li>
                            <li>{"i skipped highschool and never got a diploma."}</li>
                        </ul>

                        <p>{"if you think my projects (or me) are interesting please don't hesitate to contact me! the best way to contact me is probably through discord: bilal2vec#8916"}</p>
                        <p>{"alternatively, send me an email "} <a href="mailto:bilal.khan1@uwaterloo.ca">{"bilal.khan1@uwaterloo.ca"}</a>  {" or look me up on my "} <a href="https://linktr.ee/bkkaggle">{"other online profiles"}</a>{" (i go by @bilal2vec on most online platforms)."}</p>
                    </div>
                </div>
            </>
        }
    }
}
