use yew::prelude::*;

mod libs;

use libs::read_surfs;

#[function_component]
fn Content() -> HtmlResult {
    let surfs = ["Avondale", "Takanini"]
        .into_iter()
        .map(|location| read_surfs(location.to_string()).expect("waaa"))
        .collect::<Vec<_>>();

    println!("{:?}", surfs);

    Ok(html! {
        <div>{"Random UUID: "}{"69"}</div>
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
