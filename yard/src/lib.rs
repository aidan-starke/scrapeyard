use yew::prelude::*;

mod libs;

use libs::*;

const LOCATIONS: [&str; 2] = ["Avondale", "Takanini"];

#[function_component]
fn Content() -> HtmlResult {
    let surfs = LOCATIONS
        .into_iter()
        .map(|location| read_surfs(location.to_string()).unwrap())
        .collect::<Vec<_>>();

    Ok(html! {
        <div>
            {LOCATIONS.iter().enumerate().map(|(i, location)| html! {
                <section>
                    <h1>{location}</h1>

                    {surfs[i].clone().iter().map(|surf| html! {
                        <SurfComponent ..surf.clone() />
                    }).collect::<Html>()}
                </section>
            }).collect::<Html>()}
        </div>
    })
}

#[function_component]
fn SurfComponent(props: &Surf) -> Html {
    html! {
        <div>
            <h2>{&props.model}</h2>
            <p>{"In stock - "}{&props.count}</p>
            <ul>
                {props.links.iter().enumerate().map(|(i, link)| html! {
                    <li><a href={link.clone()}>{"Link "} {i + 1}</a></li>
                }).collect::<Html>()}
            </ul>
        </div>
    }
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
