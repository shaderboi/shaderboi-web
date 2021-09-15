use crate::components::buttons::circle_icon::CircleIcon;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct SocialSection {
    pub icon_name: String,
    pub url: String,
    pub caption: String,
    pub color: Option<String>,
}

#[derive(Properties, Clone)]
pub struct SocialSectionProps {
    pub icon_name: String,
    pub url: String,
    pub caption: String,
    pub color: Option<String>,
}

impl Component for SocialSection {
    type Message = ();
    type Properties = SocialSectionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            icon_name: props.icon_name,
            color: props.color,
            caption: props.caption,
            url: props.url,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <a href={self.url.clone()} class="section-social-media mb-1">
                <CircleIcon icon_name={self.icon_name.clone()} color={self.color.clone()} />
                <p class="size-text-regular caption-social-media text-on-background-primary">{self.caption.clone()}</p>
            </a>
        }
    }
}
