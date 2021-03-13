use bracket_terminal::prelude::*;
use kowas::bracket_views::{Input, View};

struct ImageInfo {
    image: XpFile,
    dim: (usize, usize)
}

impl ImageInfo {
    fn new(image: XpFile) -> Self{
        let dim = (
            image.layers.iter().map(|l| l.width).max().unwrap_or(0),
            image.layers.iter().map(|l| l.height).max().unwrap_or(0)
        );
        ImageInfo {
            image: image,
            dim: dim
        }
    }
}

pub struct FancyLogoMenuChoice<S, A> {
    pub text: String,
    pub key: char,
    pub enabled: bool,
    pub handler: Box<dyn Fn (&mut S) -> Option<A>>
}

pub struct InternalFancyLogoMenuChoice<S, A> {
    text: String,
    key: char,
    key_lower: char,
    key_i: Option<usize>,
    enabled: bool,
    handler: Box<dyn Fn (&mut S) -> Option<A>>
}

pub struct FancyLogoMenuView<S, A> {
    bg_col: RGB,
    title_col: RGB,
    warning_col: RGB,
    choice_col: RGB,
    choice_disabled_col: RGB,
    choice_key_col: RGB,
    choice_hover_col: RGB,
    choice_hover_glyph: FontCharType,
    logo_image: Option<ImageInfo>,
    title: String,
    choices: Vec<InternalFancyLogoMenuChoice<S, A>>,
    warnings: Vec<String>
}

impl <S, A> FancyLogoMenuView<S, A> {
    pub fn new(title: String, logo: Option<XpFile>, choices: Vec<FancyLogoMenuChoice<S, A>>, warnings: Vec<String>) -> Self {
        let internal_choices = choices.into_iter().map(|choice| {
            let key_i = choice.text.find(choice.key);
            let mut key_lower = choice.key.clone();
            key_lower.make_ascii_lowercase();
            InternalFancyLogoMenuChoice {
                text: choice.text,
                key: choice.key,
                key_lower: key_lower,
                key_i: key_i,
                enabled: choice.enabled,
                handler: choice.handler
            }
        }).collect();

        FancyLogoMenuView {
            bg_col: RGB::named(BLACK),
            title_col: RGB::from_u8(0xff, 0x00, 0x00),
            warning_col: RGB::from_u8(0x7e, 0x80, 0x7f),
            choice_col: RGB::from_u8(0xdd, 0x00, 0x00),
            choice_disabled_col: RGB::from_u8(0x66, 0x00, 0x00),
            choice_key_col: RGB::from_u8(0xff, 0x44, 0x00),
            choice_hover_col: RGB::from_u8(0xff, 0x00, 0x00),
            choice_hover_glyph: 175,
            logo_image: logo.map(|i| ImageInfo::new(i)),
            title: title,
            choices: internal_choices,
            warnings: warnings
        }
    }
}

impl <S, A> FancyLogoMenuView<S, A> {
    fn draw(&mut self, ctx: &mut BTerm) -> Option<usize> {
        let (dim_x, dim_y) = ctx.get_char_size();
        let menu_start_y = dim_y / 2 - (self.choices.len() * 2 - 1) as u32 / 2;
        let mut title_y = menu_start_y;
        let warnings_start_y = dim_y - self.warnings.len() as u32 - 1;

        if let Some(img) = &self.logo_image {
            let (img_dim_x, img_dim_y) = img.dim;
            let x =
                if img_dim_x as u32 <= dim_x { (dim_x - img_dim_x as u32) / 2 }
                else { 0 };
            let y =
                if img_dim_y as u32 <= dim_y { (dim_y - img_dim_y as u32) / 2 }
                else { 0 };
            title_y = if y >= 2 { y - 2 } else { 0 };
            ctx.render_xp_sprite(&img.image, x as i32, y as i32);
        }

        ctx.print_color_centered(title_y, self.title_col, self.bg_col, &self.title);

        let menu_right_x = dim_x / 2;
        let mut y = menu_start_y;
        for choice in self.choices.iter() {
            let col = if choice.enabled { self.choice_col } else { self.choice_disabled_col };
            ctx.print_color_right(menu_right_x, y, col, self.bg_col, choice.text.to_string());
            if let Some(i) = choice.key_i {
                let key_col = if choice.enabled { self.choice_key_col } else { self.choice_disabled_col };
                ctx.print_color(menu_right_x - (choice.text.len() - i) as u32, y, key_col, self.bg_col, choice.key);
            }
            y += 2;
        }

        let mut hover_choice_i = None;
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        let hover_y = mouse_y - menu_start_y as i32;
        if mouse_x < menu_right_x as i32 && hover_y >= 0 && hover_y % 2 == 0 {
            let hover_i = hover_y / 2;
            if hover_i < self.choices.len() as i32 {
                let left_x = menu_right_x - self.choices[hover_i as usize].text.len() as u32;
                if mouse_x >= left_x as i32 {
                    hover_choice_i = Some(hover_i as usize);
                    if self.choices[hover_i as usize].enabled {
                        let y = menu_start_y + hover_i as u32 * 2;
                        ctx.set(menu_right_x + 1, y, self.choice_hover_col, self.bg_col, self.choice_hover_glyph);
                    }
                }
            }
        }

        let mut y = warnings_start_y;
        for warning in self.warnings.iter() {
            ctx.print_color_centered(y, self.warning_col, self.bg_col, warning);
            y += 1;
        }

        hover_choice_i
    }
}

impl <S, K, I: Input<K>, A> View<S, K, I, A> for FancyLogoMenuView<S, A> {
    fn tick(&mut self, state: &mut S, _input: &I, ctx: &mut BTerm) -> Option<A> {
        ctx.cls();
        let hover_choice_i = self.draw(ctx);

        if ctx.left_click {
            if let Some(hover_i) = hover_choice_i {
                if self.choices[hover_i].enabled {
                    return (self.choices[hover_i].handler)(state)
                }
            }
        }

        if let Some(key) = ctx.key {
            let key_int = 97 + letter_to_option(key);
            if key_int > 0 {
                if let Some(mut key_char) = std::char::from_u32(key_int as u32) {
                    key_char.make_ascii_lowercase();
                    for choice in self.choices.iter() {
                        if choice.enabled && choice.key_lower == key_char {
                            return (choice.handler)(state)
                        }
                    }
                }
            }
        }

        None
    }
}
