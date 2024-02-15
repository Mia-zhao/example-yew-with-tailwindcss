use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
            <div>
                <header>
                    <nav>
                        <button>{"Business"}</button>
                        <div><button>{"ABOUT"}</button><button>{"WORK"}</button><button>{"TEAM"}</button><button>{"CONTACT"}</button></div>
                        <button>{"CONTACT US"}</button>
                    </nav>
                </header>
            <main>
                <span>{"Grow your business."}</span>
                <button>{"LEARN MORE"}</button>
            </main>
            <footer>
                <span>{"WHAT WE BELIEVE IN"}</span>
            </footer>
            </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
