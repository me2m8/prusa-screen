use leptos::{component, create_signal, view, IntoView};

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
fn HSliding_container(color: &'static str, name: &'static str, big: leptos::ReadSignal<bool>) -> impl IntoView {
    let style = format!("background-color: {}", color);

    view! {
        <div
            style=style

            class="sliding_container"
            class:container_small=move || !big()
            class:container_big=move || big()

            >
            <p class="disappearing_font name-tag">{name}</p>
            <p class="disappearing_font">"Focus"</p>
        </div>
    }
}

#[component]
fn Sliding_container_wrapper() -> impl IntoView {
    let (big1, change_size1) = create_signal(false);
    let (big2, change_size2) = create_signal(false);
    let (big3, change_size3) = create_signal(false);

    view! {
        <HSliding_container color="#ffffff" name="Ruccola" big=big1 on:click=move|_|{
            change_size1(!big1());
            change_size2(false);
        }/>
        <HSliding_container color="#0092ff" name="Information" big=big2 on:click=move|_|{
            change_size1(false);
            change_size2(!big2());
            change_size3(false);
        }/>
        <HSliding_container color="#ff9200" name="Lotta" big=big3 on:click=move|_|{
            change_size2(false);
            change_size3(!big3());
        }/>
    }
}
