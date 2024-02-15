use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
            <div class="flex flex-col h-screen">
                <header class="box-border">
                    <nav class="flex items-center justify-between my-4">
                        <button class="font-bold text-2-xl ml-8 underline-offset-8">{"Tech"}</button>
                        <div class="flex justify-between">
                            <a class="p-8 underline" href="https://yew.rs/">{"Yew"}</a>
                            <a class="p-8 underline" href="https://tailwindcss.com/">{"Tailwind CSS"}</a>
                            <a class="p-8 underline" href="https://trunkrs.dev/">{"Trunk"}</a>
                        </div>
                        <button class="bg-slate-900 text-zinc-50 py-2 px-6 mr-8">{"Try it out"}</button>
                    </nav>
                </header>
                <main class="box-border flex-1 bg-slate-900 my-10 mx-8 flex flex-col">
                    <span class="text-7xl text-zinc-50 self-center p-20 m-20">{"A Yew example with Tailwind CSS"}</span>
                    <div class="flex justify-center"><button class="text-2xl bg-zinc-50 text-slate-900 my-20 p-4">{"More Examples"}</button></div>
                </main>
                <footer class="box-border flex-none flex justify-center">
                    <span class="text-gray-600 py-4">{"Responsive Design"}</span>
                </footer>
            </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
