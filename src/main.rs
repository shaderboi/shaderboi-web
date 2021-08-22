use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
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
                <div class="header-container">
                    <div class="header-sub-container">
                        <div class="headline-container">
                            <h2>{"A place where you can find a solution for your problem and make your idea to be real."}</h2>
                            <h3>{"We realize your idea at a relatively low cost."}</h3>
                        </div>
                    </div>
                    <img class="header-sub-container header-image" src="https://shaderboi.id/img/pattern_nextjs.png" alt="Shaderboi Pattern" />
                </div>
                <div class="middle-gray">
                    <h2>{"ok"}</h2>
                </div>
                <div class="bottom-bar">
                    <svg class="fill-color-primary" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                        <polygon class="text-blueGray-200 fill-current" points="2560 0 2560 100 0 100">
                        </polygon>
                    </svg>
                    <div class="bottom-menus-container">
                        <div class="bottom-menu-container">
                        </div>
                        <div class="bottom-menu-container">
                        </div>
                        <div class="bottom-menu-container">
                            <h3>{"Useful links"}</h3>
                            <ul>
                                <li><a href="">{"Privacy Policy"}</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
