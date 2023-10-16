use std::vec;
use dioxus::prelude::*;
use crate::{MAXIMUM_WRONG_GUESSES, ResetFlag};

#[inline_props]
pub fn Hangman(cx:Scope, number_of_wrong_guesses: usize) -> Element {

    render!(rsx!(
        link { rel: "stylesheet", href: "/home/andrey/code/HangMan/input.css"}
        div {
            class: "hangman-container",

            for i in 0..MAXIMUM_WRONG_GUESSES {
                // renders another body part of the hangman for every wrong 
                // guess made by the user
                if i < *number_of_wrong_guesses {
                    rsx!(
                        div {
                            class:"hangman{i}",
                            hidden: false
                        }
                    )
                } else {
                    rsx!(
                        div {
                            class:"hangman{i}",
                            hidden: true
                        }
                    )
                }
            }          
        }

    ))
}
#[allow(non_upper_case_globals)]
#[inline_props]
pub fn KeyBoard<'a>(cx: Scope, onguess: EventHandler<'a, char>, reset_game: ResetFlag, disable_all: bool) -> Element<'a> {

    const key_board: &[&[char]] = &[
        &['q','w','e','r','t','y','u','i','o','p'],
        &['a','s','d','f','g','h','j','k','l'],
        &['z','x','c','v','b','n','m']
    ];

    let key_states = use_ref(cx, || {vec![ vec![false;10], vec![false;9] , vec![false;7]]});

    use_memo(cx, reset_game, |_reset_game| {
        key_states.set(vec![vec![false;10], vec![false;9], vec![false;7]]);
    });

    cx.render(rsx!(
        div {
            class:"keys",
            style: r#"
                display:flex;
                flex-direction:column;
                justify-content: center;
                align-items: center;
                padding:0.5rem;
            "#,
            
            for (row_number, key_row) in key_states.read().iter().enumerate() {
                rsx!(
                    div {
                        key_board[row_number].iter().enumerate().map(|(idx,key_value)| {
                            rsx!(
                                button {
                                    class: "key",
                                    disabled: *disable_all || key_row[idx],
                            onclick: move |_ev| {
                                key_states.with_mut(|states| {states[row_number][idx] = true;});
                                onguess.call(key_value.to_owned());
                            },
                            "{key_value}"
                        })
                    })
        
                })
            } 
        }
    ))
}
