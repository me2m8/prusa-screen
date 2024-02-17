use leptos::{
    component, create_rw_signal, create_signal, leptos_dom::logging::console_log, view, For,
    IntoView, RwSignal, SignalGet, SignalUpdate,
};

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
struct SlidingContainer {
    color: &'static str,
    name: &'static str,
    big: RwSignal<bool>,
    exclusive: bool,
    orientation: Orientation,
}

impl Default for SlidingContainer {
    fn default() -> Self {
        Self {
            color: Default::default(),
            name: Default::default(),
            big: Default::default(),
            exclusive: Default::default(),
            orientation: Orientation::Horizontal,
        }
    }
}

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
fn HSliding_container(
    color: &'static str,
    #[prop(optional)] name: &'static str,
    big: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div
            class="sliding_container"
            class:container_small=move||!big()
            class:container_big=big

            style:background-color=color
            >
            <p class="disappearing_font name-tag">{name}</p>
            <p class="disappearing_font">"Focus"</p>
        </div>
    }
}

#[component]
fn Sliding_container_wrapper() -> impl IntoView {
    let state = vec![
        SlidingContainer {
            name: "Ruccola",
            color: "#ffffff",
            ..Default::default()
        },
        SlidingContainer {
            name: "Information",
            color: "#0092ff",
            exclusive: true,
            ..Default::default()
        },
        SlidingContainer {
            name: "Lotta",
            color: "#ff9200",
            ..Default::default()
        },
    ];

    let (state, change_state) = create_signal(state);

    view! {
        <For
            each=move || state().into_iter().enumerate()
            key=|(_, slider)| slider.name
            children=move|(index, slider)| {
                view! {
                    <HSliding_container
                        name=slider.name
                        color=slider.color
                        big=slider.big
                        on:click=move|_|{
                            let new_state = state();

                            // Collapse all others if current slider is exclusive and
                            // current slider is small
                            if slider.exclusive && !slider.big.get() {
                                for (i, s) in new_state.iter().enumerate() {
                                    if i != index {
                                        s.big.update(|n| *n = false);
                                    }
                                }
                            }

                            new_state[index].big.update(|n| *n = !*n);
                            change_state(new_state);
                        }
                    />
                }
            }
            >
        </For>
    }
}
