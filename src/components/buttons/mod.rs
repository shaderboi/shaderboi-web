use yew_router::Switch;

pub(crate) mod circle_icon;

#[derive(Clone)]
pub(crate) enum AnchorAction<SW: Switch + Clone> {
    Href(String),
    Route(SW),
}
