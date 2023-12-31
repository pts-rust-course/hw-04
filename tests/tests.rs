use clap::Parser;
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
use solution::{Color, DynContext, EnumContext, Intersectable, Point, Shape};

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

fn to_dyn(ctx: &EnumContext) -> DynContext {
    DynContext {
        figures: ctx
            .figures
            .iter()
            .map(|f| -> Box<dyn Intersectable> {
                match f.clone() {
                    Shape::Circle(c) => Box::new(c),
                    Shape::Rectangle(r) => Box::new(r),
                    Shape::Triangle(t) => Box::new(t),
                    Shape::Background(b) => Box::new(b),
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
    triangle_tests: Vec<TestSuite>,
}

fn run_tests(tests: &[TestSuite], use_dyn: bool) {
    tests
        .iter()
        .map(|suite| {
            let ctx = &suite.ctx;
            let dyn_ctx = to_dyn(&suite.ctx);
            suite
                .points
                .iter()
                .map(|(p, c)| {
                    if use_dyn {
                        assert_eq!(solution::dyn_draw(&dyn_ctx, *p), *c)
                    } else {
                        assert_eq!(solution::enum_draw(&ctx, *p), *c)
                    }
                })
                .count()
        })
        .count();
}

fn points_test() {
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

#[derive(Debug, Clone, Parser)]
#[clap(name = "hw-test", author)]
struct Args {
    test: Option<String>,
}

macro_rules! or_if_no {
    ($e1: expr) => {
        $e1
    };
    ($e1: expr, $e2: expr) => {
        $e2
    };
}

macro_rules! tests {
    ($(=>$subtest: expr,)? $($arg:ident $(=>$func: expr)?),*) => {

        let test = Args::parse().test;

        #[allow(unused_assignments, unused_mut)]
        let mut subtest = or_if_no!("" $(,$subtest)?);

        paste::item! {
            match test.as_ref().map(|s| s.as_str()) {
                $(
                    Some(stringify!($arg)) => {
                        let f = or_if_no!([<$arg _test>] $(,$func)?);
                        test_task(f, (subtest.to_string() + " → " + stringify!($arg)).as_str())
                    },
                )*
                None => {
                    if subtest == "" {
                        println!("{}", "\n Starting tests".purple());
                    }
                    else {
                        println!("{} {}", "\n Running subtests:".purple(), subtest)
                    }

                    $(
                        if subtest != "" {print!("\t")};
                        let f = or_if_no!([<$arg _test>] $(,$func)?);
                        test_task(f, stringify!($arg));
                    )*
                },
                _ => {}
            }
        }
    };
}

#[allow(clippy::drop_copy)]
fn main() -> std::io::Result<()> {
    tests!(points);

    tests!(=> "raytracer",
        basic => || run_tests(&TESTS.simple_tests, false),
        background => || run_tests(&TESTS.background_tests, false),
        triangle => || run_tests(&TESTS.triangle_tests, false),
        dynamic => || run_tests(&TESTS.simple_tests, true));

    // Раскомментите код, чтобы порисовать картинки из набора!
    // let ctx = to_dyn(&TESTS.simple_tests[10].ctx);
    // draw_image(|p| solution::draw(&ctx, p));

    Ok(())
}

fn test_task<R, F: FnOnce() -> R + UnwindSafe>(test_func: F, test_name: &str) {
    print!("Testing {}:\t", test_name.cyan());

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
