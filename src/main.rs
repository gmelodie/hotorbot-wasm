use rand::seq::SliceRandom;
use web_sys::window;
use yew::prelude::*;

#[derive(PartialEq)]
struct ProgLang {
    name: String,
    likes: i32,
    dislikes: i32,
}

impl ProgLang {
    fn new(_name: String) -> Self {
        Self {
            name: _name,
            likes: 0,
            dislikes: 0,
        }
    }
}

#[derive(Properties, PartialEq)]
struct ProgLangProps {
    langs: Vec<ProgLang>,
}

#[function_component(RandLang)]
fn rand_lang(ProgLangProps { langs }: &ProgLangProps) -> Html {
    let lang = langs.choose(&mut rand::thread_rng()).unwrap();

    html! {
        <div class="card">
            <h2>{format!("{}", lang.name)}</h2>
            <div class="button-container">
                <button onclick={|_: MouseEvent| window().unwrap().location().reload().unwrap() } class="button dislike">{ "Not" }</button>
                <button onclick={|_: MouseEvent| window().unwrap().location().reload().unwrap() } class="button like">{ "Bot" }</button>
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    // TODO: load languages from file
    let languages = vec![
        ProgLang::new("Rust".to_owned()),
        ProgLang::new("C".to_owned()),
        ProgLang::new("C++".to_owned()),
        ProgLang::new("C#".to_owned()),
        ProgLang::new("Python".to_owned()),
        ProgLang::new("Java".to_owned()),
        ProgLang::new("JavaScript".to_owned()),
        ProgLang::new("TypeScript".to_owned()),
    ];
    html! {
        <>
        <h1>{"Hot or Bot 2"}</h1>
        <div class="container">
          <RandLang langs={languages} /> // choose random language
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
