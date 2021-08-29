use crate::components::buttons::circle_icon::CircleIcon;
use crate::components::buttons::AnchorAction;
use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub(crate) struct InfoAndButtonSection<SW: 'static + Switch + Clone, STATE: RouterState = ()> {
    pub props: Props<SW>,
    pub link: ComponentLink<Self>,
    pub router: RouteAgentDispatcher<STATE>,
}

#[derive(Properties, Clone)]
pub(crate) struct Props<SW: Switch + Clone> {
    pub button_text: String,
    pub title: String,
    pub action: AnchorAction<SW>,
    pub description: String,
}

pub(crate) enum Msg {
    Clicked,
}

impl<SW: Switch + Clone, STATE: RouterState> Component for InfoAndButtonSection<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        Self {
            props,
            link,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => match &self.props.action {
                AnchorAction::Route(sw) => {
                    let route = Route::from(sw.clone());
                    self.router.send(RouteRequest::ChangeRoute(route));
                }
                _ => {}
            },
        };
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="section-info-and-button-container mb-1">
                <div class="section-info-and-button-texts">
                    <p class="size-text-m caption-social-media text-on-primary">{self.props.title.clone()}</p>
                    <p class="size-text-s caption-social-media text-on-primary">{self.props.description.clone()}</p>
                </div>
                {
                    match &self.props.action {
                        AnchorAction::Route(sw) => {
                            let cb = self.link.callback(|event: MouseEvent| {
                                event.prevent_default();
                                Msg::Clicked
                            });
                            html!(<a onclick={cb} class="section-info-and-button-button">{self.props.button_text.clone()}</a>)
                        },
                        AnchorAction::Href(uri) =>
                            html!(<a href={uri.clone()} class="section-info-and-button-button">{self.props.button_text.clone()}</a>)
                    }
                }
            </div>
        }
    }
}
