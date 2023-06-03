use yew::prelude::*;

use crate::components::LucideIcon;

#[derive(PartialEq, Properties)]
pub struct RedirectPageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn RedirectPage(props: &RedirectPageProps) -> Html {
    let RedirectPageProps { id } = props;

    // TODO: query the actual page from backend

    html! {
      <main>
        { "Redirecting to " }
        { id }
        <LucideIcon name="loader-2" class="mr-2 h-4 w-4 animate-spin" />
      </main>
    }
}
