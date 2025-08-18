use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { "Count: {count}" }
        button { onclick: move |_| {count += 1; println!("Incremented")}, "Increment" }
        button { onclick: move |_| {count -= 1; println!("Decremented")}, "Decrement" }
        button { onclick: move |_| {count *= 0; println!("Reset")}, "reset" }
        
    }
}