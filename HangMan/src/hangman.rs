use std::vec;
use dioxus::prelude::*;

//component for managing the image of the hangman
#[inline_props]
pub fn Hangman(cx:Scope, stage: usize, reset_image: bool) -> Element {
    
    let css_status = use_ref(cx, || {vec![true;11]});    

    //changes the hidden status of the next image in queque when the user makes a guess
    use_memo(cx, stage, |stage| {
        
        css_status.with_mut(|current_list| {
            if stage >= current_list.len() {
                panic!("SHOULDNT ALLOW THE USER TO MAKE MORE THAN 10 GUESSES");
            } else {
                current_list[stage] = false;
            }
        })   
    });

    use_memo(cx, reset_image, |_| {
        css_status.with_mut(|cur| {
            *cur = vec![true;11];
        })
    });

    render!(rsx!(
        link { rel: "stylesheet", href: "HangMan/input.css"}
        div {
            class: "hangman-container",

            css_status.read().iter().skip(1).enumerate().map(move |(idx,state)| {
                rsx!(
                    div {
                        class: "hangman{(idx)}",
                        hidden: *state,
                    }
                )
            })            
        }

    ))
}
#[allow(non_upper_case_globals)]
#[inline_props]
pub fn KeyBoard<'a>(cx: Scope, onguess: EventHandler<'a, char>, reset_game: bool, disable_all: bool) -> Element<'a> {
    const row_1_letters: [char;10] = ['q','w','e','r','t','y','u','i','o','p'];
    const row_2_letters: [char;9] = ['a','s','d','f','g','h','j','k','l'];
    const row_3_letters: [char;7] = ['z','x','c','v','b','n','m'];
    
    let key_row_1 = use_ref(cx,||{
        vec![false;10]
    });

    let key_row_2 = use_ref(cx, || {
        vec![false;9]
    });

    let key_row_3 = use_ref(cx, || {
        vec![false;7]
    });

    use_memo(cx, reset_game, |_reset_game| {
        key_row_1.with_mut(|list| {*list = vec![false;10]});
        key_row_2.with_mut(|list| {*list = vec![false;9]});
        key_row_3.with_mut(|list| {*list = vec![false;7]});
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
            
            div {
                class: "key-row1",
                key_row_1.read().iter().enumerate().map(|(idx,is_disabled)| {
                    rsx!( 
                    button {
                        class: "key",
                        disabled: *is_disabled || *disable_all,
                        onclick: move |_ev| {
                            key_row_1.with_mut(|cur|{
                                cur[idx] = true; 
                            });
                            onguess.call(row_1_letters[idx]);
                        },
                        "{row_1_letters[idx]}"
                    }
                )
            })
        },

        div { 
            class: "key-row2",
            key_row_2.read().iter().enumerate().map(|(idx,is_disabled)| {
                rsx!( 
                    button {
                        class: "key",
                        disabled: *is_disabled || *disable_all,
                        onclick: move |_ev| {
                            key_row_2.with_mut(|cur|{
                                cur[idx] = true; 
                            });
                            onguess.call(row_2_letters[idx]);
                        },
                        "{row_2_letters[idx]}"
                    }
                )
            })
        },
        
        div {
            class: "key-row3",
            key_row_3.read().iter().enumerate().map(|(idx,is_disabled)| {
                rsx!( 
                    button {
                        class: "key",
                        disabled: *is_disabled || *disable_all,
                        onclick: move |_ev| {
                            key_row_3.with_mut(|cur|{
                                cur[idx] = true; 
                            });
                            onguess.call(row_3_letters[idx]);
                        },
                        "{row_3_letters[idx]}"
                    }
                )
            })
        }
        }
    ))
}
