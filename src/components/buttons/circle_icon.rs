use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct CircleIcon {
    pub icon_name: String,
    pub color: Option<String>,
}

#[derive(Properties, Clone)]
pub struct CircleIconProps {
    pub icon_name: String,
    pub color: Option<String>,
}

impl Component for CircleIcon {
    type Message = ();
    type Properties = CircleIconProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            icon_name: props.icon_name,
            color: props.color,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let style = if let Some(color) = &self.color {
            format!("color: {}", color)
        } else {
            "".to_string()
        };

        html! {
            <div class="icon-circle on-background-primary size-icon-regular">
                <i class={ self.icon_name.clone() } style={ style.clone() }></i>
            </div>
        }
    }
}
