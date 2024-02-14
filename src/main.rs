use leptos::{component, create_signal, leptos_dom::logging::console_log, view, IntoView};

fn main() {
    leptos::mount_to_body(move || view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let (big, set_big) = create_signal("");

    view! {
        <Sliding_container color="#FFFFFF" id="left" big set_big />
        <Sliding_container color="#ff9200" id="right" big set_big />
    }
}

#[component]
fn Sliding_container(
    color: &'static str,
    id: &'static str,
    big: leptos::ReadSignal<&'static str>,
    set_big: leptos::WriteSignal<&'static str>,
) -> impl IntoView {
    let style = format!("background-color: {}", color);

    view! {
        <div
            style=style

            class="sliding_container"
            class:container_small=move || id != big()
            class:container_big=move || id == big()

            on:click=move |_| {
                set_big(if id == big() {""} else {id});
                console_log("pressed");
            }
            >
            <p class="dissappearing_font">"Press when bed is cleared"</p>
        </div>
    }
}
