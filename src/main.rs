use rand::seq::SliceRandom;
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
                <button class="button">{ "Bot" }</button>
                <button class="button like">{ "Hot" }</button>
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
        <div class="container">
          <RandLang langs={languages} /> // choose random language
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
