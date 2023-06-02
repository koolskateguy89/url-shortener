use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RedirectPageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn RedirectPage(props: &RedirectPageProps) -> Html {
    let RedirectPageProps { id } = props;

    html! {
      <main>
        { "Redirecting to " }
        { id }
      </main>
    }
}
