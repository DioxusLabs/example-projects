#![allow(non_snake_case)]
use dioxus::prelude::*;
use rand::Rng;
use random_word::Lang;

mod hangman;
use crate::hangman::{KeyBoard,Hangman};

pub const MAXIMUM_WRONG_GUESSES: usize = 10;

fn main() { 
    dioxus_desktop::launch_cfg(
        Home,
        Config::default().with_window(
            WindowBuilder::new().with_title("Hangman")
        )
    );
}

fn Home(cx: Scope) -> Element {

    let counter = use_state(cx, || {0_usize});
    let game_status = use_state(cx,||{"Guess The Word!".to_string()});
    let game_ended = use_state(cx, || {false});
    let key_states = use_ref(cx, || {
        vec![ vec![false;10], vec![false;9], vec![false;7] ]
    });    let game_word = use_state(cx, || get_new_word());
    let guess_chars = use_state(cx, || {
        make_empty_guess_buffer(game_word.get().len())
    });

    
    cx.render(rsx!(     
        link {href:"input.css", rel: "stylesheet" },   
        body {     

            style: r#"
                display:flex;
                flex-direction: column;
                margin: 0;
                justify-content: center;
                align-content: space-evenly;
            "#,

            div {
                class: "main-container",
                Hangman {
                    number_of_wrong_guesses: *counter.get(),
                },
                
                p { 
                    class: "game-msg",
                    "{game_status}"
                }
                
                p { 
                    class: "hangman-word",
                    "{format_word(guess_chars.get())}" 
                },
                
                KeyBoard {
                    onguess: move |input:char| {
                        let mut current_chars= guess_chars.get().clone();
                        let mut number_of_wrong_guesses = *counter.get();
                        let chosen_word = game_word.get();
                        let mut guess_was_correct = false;
                        
                        for idx in chosen_word.contains_any(input).iter() {
                            guess_was_correct = true;
                            current_chars[*idx] = input;
                        }
                        
                        if !guess_was_correct {
                            //wrong guess
                            number_of_wrong_guesses += 1;
                            game_status.set(format!("You only have {} guesses left, be careful", MAXIMUM_WRONG_GUESSES - 1 - number_of_wrong_guesses));
                        }
                        
                        let mut game_finished = true;
                        //checks if any more characters need to be guessed
                        for ch in current_chars.iter() {
                            if *ch == '_' {
                                game_finished = false;
                                break;
                            }
                        }
                        
                        if game_finished {
                            //user won 
                            game_status.set("You Won, Click the Red Button to Play Again!".to_string());
                            game_ended.set(true);
                        }
                        
                        guess_chars.set(current_chars);
                        
                        if number_of_wrong_guesses == MAXIMUM_WRONG_GUESSES-1 {
                            //user lost
                            let real_word = game_word.get().clone();
                            let formatted = &real_word.iter().collect::<String>();
                            game_status.set(format!("You Lost, the real word was {}", formatted));
                            game_ended.set(true);
                            guess_chars.set(real_word);
                        }
                        counter.set(number_of_wrong_guesses);
                    },
                    key_states: key_states.clone(),
                    disable_all: *game_ended.get()
                },

            },
            
            button {
                class: "play-again",
                disabled: !*game_ended.get(),
                onclick: |_| {
                    key_states.set( vec![ vec![false;10], vec![false;9], vec![false;7] ] );

                    counter.set(0);

                    let new_word = get_new_word();

                    guess_chars.set(make_empty_guess_buffer(new_word.len()));
                    
                    game_word.set(new_word);

                    game_status.set("Guess the new Word".to_string());

                    game_ended.set(false);
                },
                "Play again?"
            }
        }
    ))
}

fn get_new_word() -> Vec<char> {
    //get a random word between 5 and 15 characters long
    let new_word_len: usize = rand::thread_rng().gen_range(5..15);
    //this function should never fail since there are loeads of words 5 to 15 
    //letters wrong, so its unwrap value isnt important
    let new_word = random_word::gen_len(new_word_len, Lang::En).unwrap_or("hangman");
    new_word.chars().collect::<Vec<char>>()
}

fn make_empty_guess_buffer(len: usize) -> Vec<char> {
    (0..len).map(|_| {
        '_'
    }).collect::<Vec<char>>()
}

fn format_word(word: &Vec<char>) -> String {
    let word_representation: String = word.iter().map(|ch| {
        ch.to_string() + " "
    }).collect();

    format!("{}",word_representation)
}

trait ContainsAny {
    fn contains_any(&self, compare_to: char) -> Vec<usize>;
}

impl ContainsAny for Vec<char> {
    fn contains_any(&self, compare_to: char) -> Vec<usize> {
        let mut ptrs = vec![];
        
        for (id,ch) in self.iter().enumerate() {
            if *ch == compare_to {
                ptrs.push(id);
            }
        }

        ptrs
    }
}
