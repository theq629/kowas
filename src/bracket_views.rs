use bracket_terminal::prelude::*;

pub trait Input<K> {
    fn get_mouse_pos(&self) -> Point;
    fn is_pressed(&self, key: K) -> bool;
}

pub trait View<S, K, I: Input<K>, A> {
    fn tick(&mut self, state: &mut S, input: &I, ctx: &mut BTerm) -> Option<A>;
}

pub struct ViewStack<S, K, I: Input<K>, A> {
    views: Vec<Box<dyn View<S, K, I, A>>>
}

impl <S, K, I: Input<K>, A> ViewStack<S, K, I, A> {
    pub fn new() -> Self {
        ViewStack {
            views: Vec::new()
        }
    }

    pub fn push<V: 'static + View<S, K, I, A>>(&mut self, view: V) {
        self.views.push(Box::new(view))
    }

    pub fn pop(&mut self) -> Option<Box<dyn View<S, K, I, A>>> {
        self.views.pop()
    }

    pub fn clear(&mut self) {
        self.views.clear();
    }
}

impl <S, K, I: Input<K>, A> View<S, K, I, A> for ViewStack<S, K, I, A> {
    fn tick(&mut self, state: &mut S, input: &I, ctx: &mut BTerm) -> Option<A> {
        if let Some(view) = &mut self.views.last_mut() {
            view.tick(state, input, ctx)
        } else {
            None
        }
    }
}
