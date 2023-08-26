use libs::libs;
use yew::prelude::*;

#[function_component]
fn Content() -> HtmlResult {
    let [avondale, takanini] =
        ["Avondale", "Takanini"].map(|location| match libs::read_surfs(location.to_string()) {
            Ok(surfs) => surfs,
            Err(err) => panic!("{}", err),
        });

    Ok(html! {
        <div>{"yeet: "}</div>
    })
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
