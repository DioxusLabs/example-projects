#![allow(non_snake_case)]

use std::{vec, fs};
use dioxus::prelude::*;

mod hangman;
use dioxus_desktop::{Config, WindowBuilder, tao::{dpi::{Size, Pixel}, window::Icon}, PhysicalSize};
use rand::Rng;
use random_word::Lang;

use crate::hangman::{KeyBoard,Hangman};

pub const MAXIMUM_WRONG_GUESSES: usize = 10;

fn main() {  
    // let ico = fs::read("/home/andrey/code/HangMan/public/AppImage.jpeg").expect("ICON PATH IS WRONG");
    // let w = WindowBuilder::new().with_title("Hangman")
    //     .with_window_icon(Some(Icon::from_rgba(ico,256,256).expect("icon failed")));
    // let cfg = Config::new().with_window(w);
    dioxus_desktop::launch(Home);
}

fn Home(cx: Scope) -> Element {

    let mut counter = use_state(cx, || {0_usize});
    let game_status = use_state(cx,||{"Guess The Word!".to_string()});
    let game_ended = use_state(cx, || {false});
    let reset_flag = use_state(cx, || {false});
    let game_word = use_state(cx, || get_new_word());
    let guess_chars = use_state(cx, || {
        make_empty_guess_buffer(game_word.get().len())
    });

    
    cx.render(rsx!(     
        link {href:"HangMan/input.css", rel: "stylesheet" },   
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
                    stage: *counter.get(),
                    reset_image: *reset_flag.get()
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
                        let chosen_word = game_word.get();
                        let mut set_new = false;
                        
                        for idx in chosen_word.contains_any(input).iter() {
                            set_new = true;
                            current_chars[*idx] = input;
                        }
                        
                        if !set_new {
                            //wrong guess
                            game_status.set(format!("You only have {} guesses left, be careful", MAXIMUM_WRONG_GUESSES - 1 - *counter.get()));
                            counter += 1;
                        }
                        
                        let mut game_finished = true;
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
                        
                        if *counter.get() == MAXIMUM_WRONG_GUESSES-1 {
                            //user lost
                            let real_word = game_word.get().clone();
                            let formatted = &real_word.iter().collect::<String>();
                            game_status.set(format!("You Lost, the real word was {}", formatted));
                            game_ended.set(true);
                            guess_chars.set(real_word);
                        }
                        
                    },
                    reset_game: *reset_flag.get(),
                    disable_all: *game_ended.get()
                },

            },
            
            button {
                class: "play-again",
                disabled: !*game_ended.get(),
                onclick: |_| {
                    //sets all states back to original value
                    reset_flag.set(!*reset_flag.get());
                    counter.set(0);

                    let new_word = get_new_word();
                    guess_chars.modify(|_| {
                        make_empty_guess_buffer(new_word.len())
                    });
                    
                    game_word.modify( move |_| {
                        new_word
                    });

                    game_status.set("Guess the new Word".to_string());

                    game_ended.set(false);
                },
                "Play again?"
            }
        }
    ))
}

fn get_new_word() -> Vec<char> {
    //gets a random word that is 5 to 15 characters long
    let new_word_len: usize = rand::thread_rng().gen_range(5..15);
    let new_word = random_word::gen_len(new_word_len, Lang::En).unwrap_or("ummmmmmm");
    new_word.chars().collect::<Vec<char>>()
}

fn make_empty_guess_buffer(len: usize) -> Vec<char> {
    (0..len).map(|_| {
        '_'
    }).collect::<Vec<char>>()
}

fn format_word(word: &Vec<char>) -> String {
    let str_representation: String = word.iter().map(|ch| {
        ch.to_string() + " "
    }).collect();

    format!("{}",str_representation)
}

trait ContainsAny {
    fn contains_any(&self, compare_to: char) -> Vec<usize>;
}

//searches for all indexes in char vec which contain a value
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
