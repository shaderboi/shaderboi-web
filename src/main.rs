#![recursion_limit = "256"]

pub(crate) mod components;
pub(crate) mod routes;
pub(crate) mod screen;

use components::social_section::SocialSection;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::screen::data_privacy::data_privacy::DataPrivacyScreen;
use crate::screen::data_privacy::privacy_policy::PrivacyPolicyScreen;
use crate::screen::home::HomeScreen;
use crate::screen::data_privacy::data_removal::DataRemovalScreen;

#[derive(Clone, Switch)]
pub(crate) enum RootRoute {
    #[to = "/"]
    HomeRouter,
}

#[derive(Clone, Switch)]
pub(crate) enum HomeRoute {
    #[to = "/data-privacy"]
    DataPrivacyRouter,
    #[to = "/"]
    Home,
}

#[derive(Clone, Switch)]
pub(crate) enum DataPrivacyRoute {
    #[to = "/data-privacy/privacy-policy"]
    PrivacyPolicy,
    #[to = "/data-privacy/data-removal"]
    DataRemoval,
    #[to = "/data-privacy"]
    Home,
}

pub(crate) struct DataPrivacyNavigator {
    link: ComponentLink<Self>,
}

impl Component for DataPrivacyNavigator {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            <>
                <Router<DataPrivacyRoute>
                    render = Router::render(|switch: DataPrivacyRoute| {
                        match switch {
                            DataPrivacyRoute::Home => html!{<DataPrivacyScreen />},
                            DataPrivacyRoute::PrivacyPolicy => html!{<PrivacyPolicyScreen />},
                            DataPrivacyRoute::DataRemoval => html!{<DataRemovalScreen />},
                        }
                    })
                />
            </>
        }
    }
}

pub(crate) struct HomeNavigator {
    link: ComponentLink<Self>,
}

impl Component for HomeNavigator {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            <>
                <Router<HomeRoute>
                    render = Router::render(|switch: HomeRoute| {
                        match switch {
                            HomeRoute::Home => html!{<HomeScreen />},
                            HomeRoute::DataPrivacyRouter => html!{<DataPrivacyNavigator />},
                        }
                    })
                />
            </>
        }
    }
}

pub(crate) struct Root {
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
            <>
                <div>
                    <Router<RootRoute, ()>
                        render = Router::render(|switch: RootRoute| {
                            match switch {
                                RootRoute::HomeRouter => html!{<HomeNavigator />},
                            }
                        })
                    />
                </div>
                <div class="bottom-bar">
                    <svg class="fill-color-primary" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0" style="height: 100px; width: 100%;">
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
                                <li><RouterAnchor<DataPrivacyRoute>
                                    route={DataPrivacyRoute::Home}
                                >{"Data & Privacy"}</RouterAnchor<DataPrivacyRoute>></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
