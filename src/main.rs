#![recursion_limit = "640"]
mod app_router;
mod components;
mod pages;

use yew::prelude::*;

use app_router::AppRouter;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

pub fn main() {
    App::<Model>::new().mount_to_body();
}
