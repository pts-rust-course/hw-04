pub enum Color {
    // TODO:
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Hit {
    pub color: Color,
    pub depth: i32,
}

// Реализуйте этот трейт для всех ниже определенных фигур
pub trait Intersect {
    fn intersect(&self, x: i32, y: i32) -> Option<Hit>;
}

pub struct Circle {
    pub center: Point<i32>,
    pub radius: u32,
    pub color: Color,
    pub depth: i32,
}

pub struct Rectangle {
    pub low_left: Point<i32>,
    pub high_right: Point<i32>,
    pub color: Color,
    pub depth: i32,
}

pub struct Triangle {
    pub fst: Point<i32>,
    pub snd: Point<i32>,
    pub trd: Point<i32>,
    pub color: Color,
    pub depth: i32,
}

pub struct Background {
    pub color: Color,
    pub depth: i32, // TODO: Is it really need it?
}

pub struct Context {
    pub figures: Vec<Box<dyn Intersect>>,
}

// Для закраски пиксиля выбирается цвет фигуры, которая затрагивает точку (x, y)
// и обладает наименьшей глубиной.
pub fn draw(_ctx: &Context, _x: i32, _y: i32) -> Color {
    todo!()
}

pub fn main() {}
