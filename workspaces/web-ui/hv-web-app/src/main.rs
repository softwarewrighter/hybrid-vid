use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let blocks = hv_web_core::sample_blocks();
    html! {
        <div style="font-family: sans-serif; padding: 1rem;">
            <h1>{"Hybrid-VID"}</h1>
            <p>{"Compose and run processing blocks (prototype)"}</p>
            <h2>{"Available Blocks"}</h2>
            <ul>
                { for blocks.iter().map(|b| html!{ <li>{format!("{} ({})", b.name, b.id)}</li> }) }
            </ul>
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).ok();
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

