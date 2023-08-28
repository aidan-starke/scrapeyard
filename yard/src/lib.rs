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

    if surfs.iter().any(|surfs| surfs.is_empty()) {
        return Ok(html! {
            <main>
                <h1 class="sad-life">{"No surfs found :("}</h1>
            </main>
        });
    }

    Ok(html! {
        <main>
            {LOCATIONS.iter().enumerate().map(|(i, location)| html! {
                <section>
                    <h1 class="location">{location}</h1>

                    <div class="wrapper">
                        <section class="card">
                            {surfs[i].clone().iter().map(|surf| html! {
                                <SurfComponent ..surf.clone() />
                            }).collect::<Html>()}
                        </section>
                    </div>
                </section>
            }).collect::<Html>()}
        </main>
    })
}

#[function_component]
fn SurfComponent(props: &Surf) -> Html {
    html! {
        <div class="surf">
            <h2>{&props.model}</h2>
            <p>{&props.count}{" in stock"}</p>
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
