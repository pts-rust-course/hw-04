use colored::Colorize;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use std::{
    io::BufReader,
    panic::{self, UnwindSafe},
};

#[allow(unused_imports)]
use hw_04::solution::{self, draw::draw_image};

use panic_message::panic_info_message;
use solution::{Color, Context, EnumContext, Figures, Intersectable, Point};

// Common

#[allow(unused_macros)]
macro_rules! assert {
    ($e:expr, $m: expr) => {
        if !$e {
            println!("{}: {}", "Test not passed".red(), $m.bright_red());
            panic!("test failed")
        }
    };
}

macro_rules! assert_eq {
    ($e1:expr, $e2: expr) => {
        if $e1 != $e2 {
            println!(
                "{}. {} {:?}, {} {:?}",
                "Test not passed".red(),
                "expected answer".bright_red(),
                $e1,
                "found".bright_red(),
                $e2
            );
            panic!("test failed")
        }
    };
}

// end

lazy_static! {
    static ref TESTS: Tests = {
        let file = std::fs::File::open("res/raytrace_tests.json").unwrap();
        let br = BufReader::new(file);
        serde_json::from_reader(br).unwrap()
    };
}

fn to_dyn(ctx: &EnumContext) -> Context {
    Context {
        figures: ctx
            .figures
            .iter()
            .map(|f| -> Box<dyn Intersectable> {
                match f.clone() {
                    Figures::Circle(c) => Box::new(c),
                    Figures::Rectangle(r) => Box::new(r),
                    Figures::Triangle(t) => Box::new(t),
                    Figures::Background(b) => Box::new(b),
                }
            })
            .collect(),
    }
}

#[derive(Deserialize)]
struct TestSuite {
    ctx: EnumContext,
    points: Vec<(Point<i32>, Color)>,
}

#[derive(Deserialize)]
struct Tests {
    simple_tests: Vec<TestSuite>,
    background_tests: Vec<TestSuite>,
}

fn run_tests(tests: &[TestSuite]) {
    tests
        .iter()
        .map(|suite| {
            let ctx = to_dyn(&suite.ctx);
            suite
                .points
                .iter()
                .map(|(p, c)| assert_eq!(solution::draw(&ctx, *p), *c))
                .count()
        })
        .count();
}

fn test_points() {
    let mut rng = StdRng::seed_from_u64(138);
    for _ in 0..1000 {
        let x = rng.gen_range(-1000..1000);
        let x2 = rng.gen_range(-1000..1000);
        let y = rng.gen_range(-1000..1000);
        let y2 = rng.gen_range(-1000..1000);

        assert_eq!(
            Point { x, y } + Point { x: x2, y: y2 },
            Point {
                x: x + x2,
                y: y + y2
            }
        );
        assert_eq!(
            Point { x, y } - Point { x: x2, y: y2 },
            Point {
                x: x - x2,
                y: y - y2
            }
        );
        assert_eq!(Point { x, y }.dot(Point { x: x2, y: y2 }), x * x2 + y * y2);
        assert_eq!(Point { x, y }.square(), x * x + y * y);
    }
}

fn test_simple() {
    run_tests(&TESTS.simple_tests)
}

fn test_background() {
    run_tests(&TESTS.background_tests)
}

#[allow(clippy::drop_copy)]
fn main() -> std::io::Result<()> {
    test_task(test_points, "points");
    test_task(test_simple, "2d raytracer: basic tests");
    test_task(test_background, "2d raytracer: added background");

    // Раскоммитте код, чтобы порисовать картинки из набора!
    // let ctx = to_dyn(&TESTS.simple_tests[10].ctx);
    // draw_image(|p| solution::draw(&ctx, p));

    Ok(())
}

fn test_task<R, F: FnOnce() -> R + UnwindSafe>(test_func: F, test_name: &str) {
    print!("Testing {}: ", test_name.cyan());

    panic::set_hook(Box::new(|info| {
        let msg = panic_info_message(info);
        match msg {
            // student has todo! in function
            "not yet implemented" => println!("{}", "task not complete, skipping".yellow()),
            "test failed" => {} // ignore, this is our check that is printed seperately
            // student's code paniced
            _ => println!(
                "{} '{}', {}",
                "test panicked at".red(),
                msg.red(),
                info.location().unwrap()
            ),
        }
    }));

    let r = panic::catch_unwind(test_func);
    if r.is_ok() {
        println!("{}", "test passed".green())
    }
}
