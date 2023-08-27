use std::ops::{Add, Sub};

use serde::Deserialize;

use self::draw::draw_image;

pub mod draw;

// Этот enum нельзя менять. Нельзя добавлять и убирать цвета.
// Однако МОЖНО добавлять к нему #[derive(...)].

#[derive(Clone, Default, Deserialize, PartialEq, Debug)]
pub enum Color {
    // TODO:
    #[default]
    Black,
    Green,
    Blue,
    Yellow,
    White,
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Default, Deserialize)]
pub struct RayProperties {
    pub color: Color,
    pub depth: i32,
}

// Реализуйте этот трейт для всех ниже определенных фигур
pub trait Intersectable {
    fn intersect(&self, point: Point<i32>) -> Option<RayProperties>;
}

#[derive(Clone, Deserialize)]
pub struct Circle {
    pub center: Point<i32>,
    pub radius: u32,
    pub prop: RayProperties,
}

#[derive(Clone, Deserialize)]
pub struct Rectangle {
    pub bottom_left: Point<i32>,
    pub top_right: Point<i32>,
    pub prop: RayProperties,
}

#[derive(Clone, Deserialize)]
pub struct Triangle {
    pub p1: Point<i32>,
    pub p2: Point<i32>,
    pub p3: Point<i32>,
    pub prop: RayProperties,
}

#[derive(Clone, Deserialize)]
pub struct Background {
    pub prop: RayProperties,
}

// Что мы хотим отрисовать
pub struct DynContext {
    pub figures: Vec<Box<dyn Intersectable>>,
}

// Или можно то же самое записать в другом представлении:
#[derive(Clone, Deserialize)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Background(Background),
}

#[derive(Deserialize)]
pub struct EnumContext {
    pub figures: Vec<Shape>,
}

pub fn enum_draw(_ctx: &EnumContext, _point: Point<i32>) -> Color {
    todo!()
}

// Для закраски пиксиля выбирается цвет фигуры, которая затрагивает точку (x, y)
// и обладает наименьшей глубиной.
pub fn dyn_draw(_ctx: &DynContext, _point: Point<i32>) -> Color {
    todo!()
}

impl<T> Add for Point<T> {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Sub for Point<T> {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Point<T> {
    pub fn dot(self, _rhs: Self) -> T {
        todo!()
    }

    pub fn square(self) -> T {
        todo!()
    }

    pub fn sign(self, _p2: Self, _p3: Self) -> T {
        todo!()
    }
}

impl Intersectable for Circle {
    fn intersect(&self, _point: Point<i32>) -> Option<RayProperties> {
        todo!()
    }
}

impl Intersectable for Rectangle {
    fn intersect(&self, _point: Point<i32>) -> Option<RayProperties> {
        todo!()
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, _point: Point<i32>) -> Option<RayProperties> {
        todo!()
    }
}

impl Intersectable for Background {
    fn intersect(&self, _: Point<i32>) -> Option<RayProperties> {
        todo!()
    }
}

pub fn main() {
    // Можно добавлять сюда фигуры, и при запуске `cargo run`
    // они будут отрисовываться в корне проекта в image.png
    let ctx = EnumContext {
        figures: vec![
            Shape::Circle(Circle {
                center: Point { x: 50, y: 50 },
                radius: 50,
                prop: RayProperties {
                    color: Color::Green,
                    depth: 0,
                },
            }
        )],
    };
    draw_image(|p| enum_draw(&ctx, p));
}
