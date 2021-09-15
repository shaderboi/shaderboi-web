use crate::components::info_and_button_section::InfoAndButtonSection;
use crate::components::social_section::SocialSection;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct DataRemovalScreen {
    link: ComponentLink<Self>,
}

impl Component for DataRemovalScreen {
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
            <div class="container-base container-middle-primary">
                <div class="text-on-background-primary container-content-center container-clamp-width">
                    <h1 class="text-vspacing">{"Data removal"}</h1>
                    <p>{"For data removal request, please send us an email to contact@shaderboi.id ."}</p>
                </div>
            </div>
        }
    }
}
