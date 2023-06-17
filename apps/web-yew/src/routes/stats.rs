use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StatsPageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn StatsPage(props: &StatsPageProps) -> Html {
    let StatsPageProps { id } = props;

    // TODO: fetch stats from server

    let url = "placeholder";
    let hits = 0; // placeholder

    html! {
        <main>
            { id }
            <h1>{ "Stats" }</h1>
            <p>{ "URL:" } {url}</p>
            <p>{ "Hits:" } {hits}</p>
        </main>
    }
}
