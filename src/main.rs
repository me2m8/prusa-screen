use leptos::{component, create_signal, leptos_dom::logging::console_log, view, IntoView};

fn main() {
    leptos::mount_to_body(move || view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Sliding_container_wrapper />
    }
}

#[component]
fn Sliding_container(color: &'static str, name: &'static str) -> impl IntoView {
    let (big, change_size) = create_signal(false);

    let style = format!("background-color: {}", color);

    view! {
        <div
            style=style

            class="sliding_container"
            class:container_small=move || !big()
            class:container_big=move || big()

            on:click=move |_| {
                change_size(!big());
                console_log("pressed");
            }
            >
            <p class="disappearing_font name-tag">{name}</p>
            <p class="disappearing_font">"Focus"</p>
        </div>
    }
}

#[component]
fn Sliding_container_wrapper() -> impl IntoView {
    view! {
        <Sliding_container color="#ffffff" name="Ruccola"/>
        <Sliding_container color="#0092ff" name="middle"/>
        <Sliding_container color="#ff9200" name="Lotta"/>
    }
}
