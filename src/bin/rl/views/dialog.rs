use std::cmp::min;
use bracket_terminal::prelude::*;
use sevendrl_2021::bracket_views::{Input, View};

pub struct DialogChoice<S, A> {
    pub text: String,
    pub key: char,
    pub handler: Box<dyn Fn (&mut S) -> Option<A>>
}

struct InternalDialogChoice<S, A> {
    text: String,
    key: char,
    key_lower: char,
    key_i: Option<usize>,
    pub handler: Box<dyn Fn (&mut S) -> Option<A>>
}

pub struct DialogView<S, A> {
    bg_col: RGB,
    title_col: RGB,
    text_col: RGB,
    choice_col: RGB,
    choice_key_col: RGB,
    choice_hover_col: RGB,
    choice_hover_glyph: FontCharType,
    title: String,
    text: String,
    default: Option<usize>,
    choices: Vec<InternalDialogChoice<S, A>>
}

impl <S, A> DialogView<S, A> {
    pub fn new(title: String, text: String, default: Option<usize>, choices: Vec<DialogChoice<S, A>>) -> Self {
        let internal_choices = choices.into_iter().map(|choice| {
            let key_i = choice.text.find(choice.key);
            let mut key_lower = choice.key.clone();
            key_lower.make_ascii_lowercase();
            InternalDialogChoice {
                text: choice.text,
                key: choice.key,
                key_lower: key_lower,
                key_i: key_i,
                handler: choice.handler
            }
        }).collect();
        DialogView {
            bg_col: RGB::named(BLACK),
            title_col: RGB::from_u8(0xff, 0x00, 0x00),
            text_col: RGB::from_u8(0xd9, 0xdd, 0xda),
            choice_col: RGB::from_u8(0xdd, 0x00, 0x00),
            choice_key_col: RGB::from_u8(0xff, 0x44, 0x00),
            choice_hover_col: RGB::from_u8(0xff, 0x00, 0x00),
            choice_hover_glyph: 175,
            title: title.to_string(),
            text: text.to_string(),
            default: default,
            choices: internal_choices
        }
    }

    fn draw(&mut self, ctx: &mut BTerm) -> Option<usize> {
        let (dim_x, dim_y) = ctx.get_char_size();
        let text_width = min(32, dim_y - 2);
        let text_left_x = (dim_x - text_width) / 2;
        let choices_max_size = self.choices.iter().map(|c| c.text.len()).max().unwrap_or(0);
        let choices_left_x = (dim_x - choices_max_size as u32 - 2) / 2;
        let choices_right_x = choices_left_x + choices_max_size as u32;
        let choices_start_y = dim_y - self.choices.len() as u32 - 1;

        let mut y = 1;

        ctx.print_color_centered(y, self.title_col, self.bg_col, &self.title);
        y += 2;

        let lines = textwrap::wrap(&self.text, text_width as usize);
        for line in lines {
            ctx.print_color(text_left_x, y, self.text_col, self.bg_col, line);
            y += 1;
        }

        let (mouse_x, mouse_y) = ctx.mouse_pos();
        let mut hover_choice_i = None;
        if mouse_x >= choices_left_x as i32 && mouse_x < choices_right_x as i32 {
            let hover_y = mouse_y - choices_start_y as i32;
            if hover_y >= 0 && hover_y < self.choices.len() as i32 {
                hover_choice_i = Some(hover_y as usize)
            }
        }

        let mut y = dim_y - self.choices.len() as u32 - 1;
        for choice in self.choices.iter() {
            ctx.print_color(choices_left_x, y, self.choice_col, self.bg_col, &choice.text);
            if let Some(i) = choice.key_i {
                ctx.set(choices_left_x + i as u32, y, self.choice_key_col, self.bg_col, to_cp437(choice.key));
            }
            y += 1;
        }
        if let Some(hover_i) = hover_choice_i {
            let y = choices_start_y + hover_i as u32;
            ctx.set(choices_right_x + 1, y, self.choice_hover_col, self.bg_col, self.choice_hover_glyph);
        }

        hover_choice_i
    }
}

impl <S, K, I: Input<K>, A> View<S, K, I, A> for DialogView<S, A> {
    fn tick(&mut self, state: &mut S, _input: &I, ctx: &mut BTerm) -> Option<A> {
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();
        let hover_choice_i = self.draw(ctx);

        let mut chosen = None;

        if let Some(i) = self.default {
            match ctx.key {
                Some(VirtualKeyCode::Return) => {
                    chosen = Some(&self.choices[i]);
                },
                Some(VirtualKeyCode::Space) => {
                    chosen = Some(&self.choices[i]);
                },
                _ => {}
            }
        }

        if let Some(key) = ctx.key {
            let key_int = 97 + letter_to_option(key);
            if key_int > 0 {
                if let Some(mut key_char) = std::char::from_u32(key_int as u32) {
                    key_char.make_ascii_lowercase();
                    for choice in self.choices.iter() {
                        if choice.key_lower == key_char {
                            chosen = Some(choice);
                        }
                    }
                }
            }
        }

        if ctx.left_click {
            if let Some(i) = hover_choice_i {
                chosen = Some(&self.choices[i]);
            }
        }

        if let Some(chosen) = chosen {
            (chosen.handler)(state)
        } else {
            None
        }
    }
}
