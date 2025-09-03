use dioxus::prelude::*;

#[component]
fn AppBasic() -> Element {
    rsx! {
    div { id: "title",
                h1 { "HotDog! 🌭" }
            }
            div { id: "dogview",
                img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
            }
            div { id: "buttons",
                button { id: "skip", "skip" }
                button { id: "save", "save!" }
            }
        }
}
#[component]
fn DogAppBasic() -> Element {
    let name = "Rex";
    rsx! {
        header{background_color: "gray",
            h1 {"Dog name"}        }
        p {"{name}"}
        footer{"Ele é um bom menino"}
    }
}

#[component]
fn DogApp() -> Element {
    let breed = "Golden Retriever";
    let dogs = vec!["Golden Retriever", "Labrador Retriever", "German Shepherd"];
    let mut show_title = use_signal(|| true);
    tracing::info!("Rendering dog app with breed: {breed}");

    rsx! {
        button { onclick: move |_| show_title.toggle(), "Toggle Title" }

                // Conditional rendering
                {show_title().then(|| rsx! { "title!" })}

                // Iterator
                ul {
                    {(0..5).map(|i| rsx! { li { "{i}" } })}
                }

    }
}

fn main() {
    // launch(DogApp);
    // launch(AppBasic);
    // launch(DogAppBasic);
}
