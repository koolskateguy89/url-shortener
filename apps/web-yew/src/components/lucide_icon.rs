use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LucideIconProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub alt: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

/// It won't use the color from the parent, because it's an image. It is black.
/// https://lucide.dev/docs/lucide-static
#[function_component]
pub fn LucideIcon(props: &LucideIconProps) -> Html {
    let LucideIconProps {
        name,
        alt: props_alt,
        class,
    } = props;

    let src = format!("https://unpkg.com/lucide-static@latest/icons/{}.svg", name);

    let alt = if props_alt.is_empty() {
        name
    } else {
        props_alt
    };

    let class = class.clone();

    html! {
        <img {src} {alt} {class} />
    }
}
