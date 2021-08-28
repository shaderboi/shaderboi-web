pub(crate) mod components;

use components::social_section::SocialSection;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub struct Root {
    link: ComponentLink<Self>,
}

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                            <h2 class="size-text-large">{"A place where you can find a solution for your problem and make your idea to be real."}</h2>
                            <h3 class="size-text-regular">{"We realize your idea at a relatively low cost."}</h3>
                        </div>
                    </div>
                    <img class="header-sub-container header-image" src="https://shaderboi.id/img/pattern_nextjs.png" alt="Shaderboi Pattern" />
                </div>
                <div class="container-middle-primary container-base container-content-center">
                    <div>
                        <h2 class="text-center size-text-large text-on-primary mb-2">{"Keep in touch with us!"}</h2>
                        <SocialSection icon_name="fab fa-whatsapp" url="https://wa.me/6281296390327" color={Some(format!("#25D366"))} caption="+62 812-9639-0327 (Michael)" />
                        <SocialSection icon_name="fab fa-line" url="http://line.me/ti/p/~mike0883" color={Some(format!("#06C755"))} caption="mike0883 (Michael)" />
                        <SocialSection icon_name="fab fa-instagram" url="https://instagram.com/shaderboi" caption="@shaderboi" />
                        <SocialSection icon_name="fab fa-facebook" url="https://facebook.com/shaderboi" color={Some(format!("#4267B2"))} caption="Shaderboi" />
                        <SocialSection icon_name="fab fa-linkedin" url="https://www.linkedin.com/company/shaderboi/" color={Some(format!("#0077B5"))} caption="Shaderboi" />
                    </div>
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
                            <h3>{"Links"}</h3>
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
    yew::start_app::<Root>();
}
