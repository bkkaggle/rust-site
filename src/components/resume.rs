use yew::prelude::*;

struct Part {
    title: String,
    role: String,
    time: String,
    items: Vec<Html>,
}

struct Section {
    name: String,
    parts: Vec<Part>,
}

fn render_part(part: &Part, expanded: &bool) -> Html {
    html! {
        <div class="part wrap">

            {
                if *expanded {
                    html! { <br /> }
                } else {
                    html! {""}
                }
            }
            <div class="flex-horizontal space-between wrap">
                <h3 class="serifheading">{&part.title}</h3>
                <h4>{&part.role} {" — "} {&part.time}</h4>
            </div>
            <br />
            {
                if *expanded {
                    part
                        .items
                        .iter()
                        .map(|item| html! {<li> {item.clone()} </li>})
                        .collect::<Html>()
                } else {
                    html!{""}
                }
            }
        </div>
    }
}

fn render_section(section: &Section, expanded: &bool) -> Html {
    html! {
        <div>
            <h2>{&section.name}</h2>
            {section.parts.iter().map(|part| render_part(part, expanded)).collect::<Html>()}
        </div>
    }
}

pub enum Msg {
    ToggleExpanded,
}

pub struct Resume {
    link: ComponentLink<Self>,
    expanded: bool,
    content: Vec<Section>,
}

impl Component for Resume {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let content = vec![
            Section {
                name: "Experience".to_string(),
                parts: vec![
                    Part {
                        title: "NLP research project".to_string(),
                        role: "Replicated OpenAI's GPT-2 model".to_string(),
                        time: "March 2020 - May 2020".to_string(),
                        items: vec![
                            html! {"I replicated OpenAI's 124 million parameter GPT-2 language model to generate convincing human-like text (my model got 21ppl compared to 17ppl for GPT-2) by pretraining it on OpenWebText (a large dataset of 40GB of text scraped from Reddit)"},
                            html! {"I pretrained the model on a 128-core TPU pod (a 6 PetaFLOP cluster of AI accelerator chips) on the Google Cloud Platform with support from the Tensorflow Research Cloud Program."},
                            html! {"If I wanted to rent the same TPU Pod myself without support from the TFRC, it would cost me about $20,000 for the one week that I got access to the cluster."},
                            html! {"Since I only had access to the pod for a week, I was only able to train it for 1/8th of the amount of iterations as OpenAI (100,000 vs 800,000), which is why my model is 4ppl worse than OpenAI's."},
                            html! {
                                <span>
                                    {"I wrote up a series of blog posts on what I learned over the three
                                    months that I was working on this: "}
                                    <a href="/2020/6/22/nlp-series-1/">{"Part 1"}</a>{" and "}
                                    <a href="/2020/6/22/nlp-series-1/">{"Part 2"}</a>
                                </span>
                            },
                            html! {
                                <span>
                                    {"My code for this project is spread across a few Github repos "}
                                    <a href="https://github.com/bkkaggle/lm-finetuning">{"here"}</a>{" and "}
                                    <a href="https://github.com/bkkaggle/lm-training-research-project">
                                        {"here"}
                                    </a>
                                </span>
                            },
                        ],
                    },
                    Part {
                        title: "SATisfied".to_string(),
                        role: "Mobile and Backend developer".to_string(),
                        time: "Sep 2019 - Now".to_string(),
                        items: vec![
                            html! {"Working on developing an app for the education space."},
                            html! {"Our stack: React Native, ExpressJS, and MySQL"},
                        ],
                    },
                    Part {
                        title: "Kaggle".to_string(),
                        role: "Highest rank: 798/124,000+".to_string(),
                        time: "Dec 2017 - July 2019".to_string(),
                        items: vec![
                            html! {
                                <span>
                                    {"I used to spend a lot of time competing in Kaggle competitions, winning "}
                                    <a href="https://kaggle.com/bkkaggle">{"a few medals along the way"}</a>{" (1 silver
                                    and 4 bronze)."}
                                </span>
                            },
                            html! {"I've competed in competitions in areas like biomedical image segmentation, toxic comment classification, and high energy physics particle tracking."},
                            html! {"I'm a competitions expert. At one point I was ranked 798th out of more than 124,000 active competitors on the Kaggle competitions leaderboard."},
                            html! {
                                <span>
                                    {"My favorite experience on Kaggle was getting 54th place in the "}
                                    <a href="https://www.kaggle.com/c/trackml-particle-identification">
                                        {"CERN TrackML Particle Tracking Challenge"}
                                    </a>
                                    {" , where I worked on reconstructing the path of particles from 3d points
                                    left in silicon detectors in a simulated version of a more powerful
                                    version of the Large Hadron Collider. I put all of my work on github "}
                                    <a href="https://github.com/bkkaggle/TrackML">{"here."}</a>
                                </span>

                            },
                        ],
                    },
                ],
            },
            Section {
                name: "Education".to_string(),
                parts: vec![
                    Part {
                        title: "University of Waterloo".to_string(),
                        role: "Software Engineering".to_string(),
                        time: "Sep 2020 - 2025".to_string(),
                        items: vec![
                            html! {"I'm an incoming 1A software engineering student at the University of Waterloo"},
                            html! {"I'm writing a series of blog posts on my unconventional path to applying to UW (teaser: I chose not to get a high school diploma), I'll post a link here when it's ready."},
                        ],
                    },
                    Part {
                        title: "Coursera".to_string(),
                        role: "Deep Learning Specialization".to_string(),
                        time: "Nov 2017 - Feb 2018".to_string(),
                        items: vec![html! {
                            <span>
                                {"I finished all 5 courses in the Coursera Deep Learning Specialization.
                                Here's the "}
                                <a href="https://www.coursera.org/account/accomplishments/specialization/8YRK7HDMW5XZ">
                                    {"verification link"}
                                </a>
                            </span>
                        }],
                    },
                ],
            },
            Section {
                name: "Projects".to_string(),
                parts: vec![
                    Part {
                        title: "Raytracer".to_string(),
                        role: "A simple raytracer book that I followed".to_string(),
                        time: "July 2020".to_string(),
                        items: vec![
                            html! {
                                <span>
                                    {"I made a simple "}
                                    <a href="https://github.com/bkkaggle/raytracer">{"ray tracer"}</a>{" following
                                    the "}
                                    <a href="https://github.com/bkkaggle/raytracer">
                                        {"Ray Tracing in One Weekend"}
                                    </a>
                                    {" book"}
                                </span>
                            },
                            html! {"It's pretty simple but was a good way for me to learn a little about computer graphics and rendering."},
                        ],
                    },
                    Part {
                        title: "L2".to_string(),
                        role: "A fast, BLAS-accelerated machine learing library".to_string(),
                        time: "May 2020 - June 2020".to_string(),
                        items: vec![
                            html! {
                                <span>
                                    <a href="https://github.com/bkkaggle/L2">{"L2"}</a>{" is an extremely fast
                                    machine learning library written in Rust. It supports most major math
                                    operations (including fast, BLAS-accelerated matrix multiplication) and
                                    has an efficient graph-based automatic differentiation engine that
                                    topologically sorts and traverses the graph to compute the gradients."}
                                </span>
                            },
                            html! {
                                <span>
                                    {"I also wrote a "} <a href="https://github.com/bkkaggle/L2/tree/c%2B%2B">{"C++ version"}</a>{" of the library back in 2019."}
                                </span>
                            },
                            html! {
                                "This was probably my favorite side project to work on. L2 is *almost* as fast as Pytorch, it can do a matmul on two 64x64 matrices in 50us while Pytorch takes 15us, which is only 3x faster — pretty impressive since I didn't try to squeeze out every last bit of performance from my library."
                            },
                            html! {
                                "Seriously, I got a 300x speedup when I switched from my own hand-rolled matmul implementation to BLAS."
                            },
                            html! {
                                "This was the first time I wrote something that was kinda complicated in Rust, and I enjoyed using the language; I'd definitely like to use Rust again soon."
                            },
                            html! {
                                <span>
                                    {"It got to the front page of "}
                                    <a href="https://lobste.rs/s/a02csy/extremely_fast_pytorch_style_tensor">
                                        {"lobste.rs"}
                                    </a>{" and stayed there for about a day."}
                                </span>
                            },
                        ],
                    },
                    Part {
                        title: "Competitive programming".to_string(),
                        role: "Solutions and notes".to_string(),
                        time: "Feb 2020".to_string(),
                        items: vec![html! {
                            <span>
                                {"My "}<a href="https://github.com/bkkaggle/ccc">{"notes and solutions"}</a>{" for
                                past CCC contests in Python and C++."}
                            </span>
                        }],
                    },
                    Part {
                        title: "Pytorch Zoo".to_string(),
                        role: "A Pytorch utility library".to_string(),
                        time: "June 2019".to_string(),
                        items: vec![
                            html! {"I built Pytorch Zoo, a Python library with a collection of useful modules and utilities for working with PyTorch."},
                            html! {
                                <span>
                                    {"I also published a blog post about the library on the "}
                                    <a href="https://medium.com/pytorch/pytorch-zoo-a-collection-of-useful-modules-and-utilities-in-pytorch-c05ca4d500d8">
                                        {"official PyTorch blog"}
                                    </a>
                                </span>
                            },
                            html! {
                                "Pytorch Zoo has 80+ stars on Github."
                            },
                            html! {
                                "I built this thinking that I'd use it for all of my projects but a year later I usually just copy over whatever function I need"
                            },
                        ],
                    },
                    Part {
                        title: "Machine Learning Flight Rules".to_string(),
                        role: "A guide".to_string(),
                        time: "Sep 2019".to_string(),
                        items: vec![
                            html! {"I wrote up a bunch of my notes on common problems that I've run into when working with machine learning."},
                            html! {
                                <span>
                                    {"You can take a look at it on the Github "}
                                    <a href="https://github.com/bkkaggle/machine-learning-flight-rules">
                                        {"repo"}
                                    </a>{" or as a "}<a href="/2020/7/3/ml-flight-rules/">{"blog post"}</a>
                                </span>
                            },
                        ],
                    },
                    Part {
                        title: "Handwritten math expression recognition".to_string(),
                        role: "Implemented papers".to_string(),
                        time: "Feb 2019 - May 2019".to_string(),
                        items: vec![
                            html! {"I worked on using neural networks to transcribe handwritten math equations into LaTeX, implementing several machine learning papers in the field (WAP, Im2Latex, etc)."},
                            html! {
                                <span>
                                    {"I put all my code for this on Github "}
                                    <a href="https://github.com/bkkaggle/math-expression-recognition">
                                        {"here"}
                                    </a>{" and "}<a href="https://github.com/bkkaggle/math-recognition">{"here"}</a>
                                </span>
                            },
                        ],
                    },
                    Part {
                        title: "Mission Control".to_string(),
                        role: "View pictures beamed back from Mars".to_string(),
                        time: "Dec 2017".to_string(),
                        items: vec![
                            html! {"I made a react.js web app that pulls down the latest images sent back from the Curiosity and Opportunity Mars rovers."},
                            html! {
                                <span>
                                    {"I'm surprised the website's still up at "}
                                    <a href="https://bilal.software/mission-control/">
                                        {"bilal.software/mission-control"}
                                    </a>{". The code is available "}
                                    <a href="https://github.com/bkkaggle/mission-control">{"here"}</a>
                                </span>
                            },
                        ],
                    },
                ],
            },
            Section {
                name: "Hobbies".to_string(),
                parts: vec![
                    Part {
                        title: "Running".to_string(),
                        role: "10 min/mile".to_string(),
                        time: "2020 - Now".to_string(),
                        items: vec![
                            html! {"Working on improving my speed (6m/km or 10m/mile) and distance (3-5k every day)"},
                        ],
                    },
                    Part {
                        title: "Writing".to_string(),
                        role: "Mostly technical blog posts".to_string(),
                        time: "2020 - Now".to_string(),
                        items: vec![
                            html! {
                                <span>
                                    {"I'm writing up a continuing"}
                                    <a href="/2020/3/25/notes-1-normalization/">{"series"}</a>{" of posts on notes that I've been making for myself about certain ML topics and papers."}
                                </span>
                            },
                            html! {
                                <span>
                                    {"I've also finished writing a long-form (~5k words) two part blog post on
                                    the NLP research project I was talking about above: "} <a href="/2020/6/22/nlp-series-1/">{"Part 1"}</a>{" and "}
                                    <a href="/2020/6/22/nlp-series-1/">{" Part 2"}</a>
                                </span>
                            },
                            html! {
                                <span>
                                    {"I'm currently (kinda) working on polishing up a really "} <a href="/2020/8/02/l2/">{"long post"}</a>{" on how to build a PyTorch-style machine learning library in Rust."}
                                </span>
                            },
                        ],
                    },
                    Part {
                        title: "Servers".to_string(),
                        role: "Self-hosting".to_string(),
                        time: "2019".to_string(),
                        items: vec![
                            html! {"I used to self-host a gitea server to host a mirror of all my github repositories and a nextcloud server as a sort of self-hosted google drive."},
                            html! {
                                "It's not up anymore because my old laptop that I use as a server is s̶l̶o̶w̶in the garbage and fixing ssl cert errors isnt fun"
                            },
                        ],
                    },
                    Part {
                        title: "Languages".to_string(),
                        role: "French, Japanese, and Chinese".to_string(),
                        time: "2016 - 2020".to_string(),
                        items: vec![
                            html! {"I finished Duolingo's French and Japanese trees and was working on Chinese."},
                            html! {"I can still speak and understand a little French and Japanese, but not very well." },
                        ],
                    },
                ],
            },
            Section {
                name: "Failures".to_string(),
                parts: vec![
                    Part {
                        title: "Rejections".to_string(),
                        role: "various".to_string(),
                        time: "2019 - 2020".to_string(),
                        items: vec![
                            // html! {"Got rejected from Waterloo's CS program in a year where they accepted 225% of their normal number of students 🤡."},
                            html! {"Couldn't apply to a whole bunch of American universities and programs that I really liked because they either require high-school diplomas or don't give any scholarships or finaid to international students."},
                            html! {"Got rejected from the one American uni that I *could* apply to that actually accepts students without diplomas 🙃."},
                            html! {"My submission was rejected from WWDC 2020 (the trip to Cupertino for the winners was cancelled too, unfortunately)"},
                            html! {"Couldn't beome a finalist for the TF repo at GCI 2019 (the trip to Google headquarters was cancelled too, courtesy of covid), but hey at least I got a t-shirt (see below 👇). Geez, 2020 really is/was cursed"},
                        ],
                    },
                    Part {
                        title: "Missing dev swag".to_string(),
                        role: "lost in s̶p̶a̶c̶e̶transit".to_string(),
                        time: "2019—?!?".to_string(),
                        items: vec![
                            html! {"According to UPS, my Hacktoberfest 2019 t-shirt was stuck in Illinois for a couple of months before the tracking link expired."},
                            html! {"My GCI t-shirt also went AWOL, probably something to do with covid delaying shipping for a couple of weeks and not having a tracking number."},
                        ],
                    },
                    Part {
                        title: "Video games".to_string(),
                        role: "That I'm not very good at".to_string(),
                        time: "not often enough".to_string(),
                        items: vec![html! {"Too many to list"}],
                    },
                ],
            },
        ];

        Self {
            link,
            expanded: false,
            content,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleExpanded => {
                self.expanded = !self.expanded;
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div id="resume">
                <div class="flex-horizontal space-between">
                    <h1>{"What I've Done"}</h1>
                    <div>
                        <div class="block accent" id="toggle" onclick=self.link.callback(|_| Msg::ToggleExpanded)>
                            {if self.expanded { "less stuff" } else { "the kitchen sink" }}
                        </div>
                    </div>
                </div>
                <div id="projects">
                    {self.content.iter().map(|part| render_section(part, &self.expanded)).collect::<Html>()}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
