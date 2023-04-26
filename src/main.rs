use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let languages = vec![
        ProgLang::new("Python".to_owned()),
        ProgLang::new("Elixir".to_owned()),
        ProgLang::new("Clojure".to_owned()),
        ProgLang::new("Julia".to_owned()),
        ProgLang::new("Bash/Shell".to_owned()),
        ProgLang::new("Go".to_owned()),
        ProgLang::new("Java".to_owned()),
        ProgLang::new("Kotlin".to_owned()),
        ProgLang::new("PHP".to_owned()),
        ProgLang::new("C#".to_owned()),
        ProgLang::new("Swift".to_owned()),
        ProgLang::new("R".to_owned()),
        ProgLang::new("Ruby".to_owned()),
        ProgLang::new("C".to_owned()),
        ProgLang::new("C++".to_owned()),
        ProgLang::new("Matlab".to_owned()),
        ProgLang::new("TypeScript".to_owned()),
        ProgLang::new("Scala".to_owned()),
        ProgLang::new("Rust".to_owned()),
        ProgLang::new("Perl".to_owned()),
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
