use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RedirectProps {
    pub id: AttrValue,
}

#[function_component(RedirectPage)]
pub fn redirect(props: &RedirectProps) -> Html {
    let RedirectProps { id } = props;

    html! {
      <main>
        { "Redirecting to " }
        { id }
      </main>
    }
}
