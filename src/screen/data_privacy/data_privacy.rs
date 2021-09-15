use crate::components::buttons::AnchorAction;
use crate::components::info_and_button_section::InfoAndButtonSection;
use crate::components::social_section::SocialSection;
use crate::DataPrivacyRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct DataPrivacyScreen {
    link: ComponentLink<Self>,
}

impl Component for DataPrivacyScreen {
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
                <div class="container-middle-primary container-base container-content-center">
                    <div>
                        <h2 class="text-center size-text-l text-on-background-primary mb-2">{"Data & Privacy"}</h2>
                        <InfoAndButtonSection<DataPrivacyRoute> button_text="View" action={AnchorAction::Route(DataPrivacyRoute::PrivacyPolicy)} title="Privacy Policy" description="Read more about how Shaderboi collects and processes your personal data." />
                        <InfoAndButtonSection<DataPrivacyRoute> button_text="Request" action={AnchorAction::Route(DataPrivacyRoute::DataRemoval)} title="Remove My Data" description="Shaderboi will remove all your personal data." />
                    </div>
                </div>
            </div>
        }
    }
}
