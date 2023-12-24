use std::ops::{Add, Mul};

use z3::{
    ast::{Ast, Int, Real},
    Config, SatResult,
};

#[derive(Debug, Clone)]
pub struct HailStone {
    x: i128,
    y: i128,
    z: i128,
    delta_x: i128,
    delta_y: i128,
    delta_z: i128,
}

pub fn parse<'a>(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .map(|l| {
            let (pos, velocity) = l.split_once(" @ ").unwrap();
            let (pos_x, pos_yz) = pos.split_once(", ").unwrap();
            let (pos_y, pos_z) = pos_yz.split_once(", ").unwrap();

            let (delta_x, delta_yz) = velocity.split_once(", ").unwrap();
            let (delta_y, delta_z) = delta_yz.split_once(", ").unwrap();

            HailStone {
                x: pos_x.trim().parse().unwrap(),
                y: pos_y.trim().parse().unwrap(),
                z: pos_z.trim().parse().unwrap(),
                delta_x: delta_x.trim().parse().unwrap(),
                delta_y: delta_y.trim().parse().unwrap(),
                delta_z: delta_z.trim().parse().unwrap(),
            }
        })
        .collect()
}

/// Coefficients in the equation: ax + by = c
#[derive(Debug, Clone)]
struct Line {
    a: i128,
    b: i128,
    c: i128,
}

impl Line {
    fn from_hailstone(stone: &HailStone) -> Self {
        // Use the current point and point point in the future.
        let x1 = stone.x;
        let y1 = stone.y;

        let x2 = x1 + stone.delta_x;
        let y2 = y1 + stone.delta_y;

        let a = y1 - y2;
        let b = x2 - x1;

        let c = -(x1 * y2 - x2 * y1);

        Line { a, b, c }
    }
}

const RANGE_MIN: f64 = 200000000000000.0;
const RANGE_MAX: f64 = 400000000000000.0;
// const RANGE_MIN: f64 = 7.0;
// const RANGE_MAX: f64 = 27.0;
pub fn solve_part_one<'a>(input: Vec<HailStone>) -> String {
    let lines: Vec<Line> = input
        .iter()
        .map(|stone| Line::from_hailstone(stone))
        .collect();

    let mut intersections = 0;
    for (i, line_1) in lines.iter().enumerate() {
        for (j, line_2) in lines.iter().enumerate() {
            if j <= i {
                // Don't compare to ourselves or to any of the ones we've already handled.
                continue;
            }

            let delta = line_1.a * line_2.b - line_2.a * line_1.b;

            if delta == 0 {
                // No intersections, they are parallel
                println!("Line {i} and {j} are paralell");
                continue;
            }

            let delta = delta as f64;
            let intersect_x = ((line_2.b * line_1.c - line_1.b * line_2.c) as f64) / delta;
            let intersect_y = ((line_1.a * line_2.c - line_2.a * line_1.c) as f64) / delta;

            if intersect_x >= RANGE_MIN
                && intersect_x <= RANGE_MAX
                && intersect_y >= RANGE_MIN
                && intersect_y <= RANGE_MAX
            {
                println!("Line {i} collides with {j} at ({intersect_x}, {intersect_y})");

                println!(
                    "\tHailstone \n\t{:?}\n\t{:?}\n\n\t{:?}\n\t{:?}",
                    input[i], input[j], lines[i], lines[j]
                );

                // Check if it happened in the past or future.
                if (input[i].delta_x < 0 && (input[i].x as f64) < intersect_x)
                    || (input[i].delta_x >= 0 && (input[i].x as f64) > intersect_x)
                {
                    println!("\tCollision happened in the past for line {i}");
                    continue;
                }

                if (input[j].delta_x < 0 && (input[j].x as f64) < intersect_x)
                    || (input[j].delta_x >= 0 && (input[j].x as f64) > intersect_x)
                {
                    println!("\tCollision happened in the past for line {j}");
                    continue;
                }
                intersections += 1;
            }
        }
    }

    intersections.to_string()
}

pub fn solve_part_two<'a>(input: Vec<HailStone>) -> String {
    let cfg = Config::new();
    let ctx = z3::Context::new(&cfg);

    let solver = z3::Solver::new(&ctx);

    let f_x = Real::new_const(&ctx, "fx");
    let f_y = Real::new_const(&ctx, "fy");
    let f_z = Real::new_const(&ctx, "fz");

    let f_dx = Real::new_const(&ctx, "fdx");
    let f_dy = Real::new_const(&ctx, "fdy");
    let f_dz = Real::new_const(&ctx, "fdz");

    // We only need the first 3.
    for i in 0..=2 {
        let s = &input[i];

        let t = Real::new_const(&ctx, format!("t{i}"));

        solver.assert(&t.ge(&Real::from_int(&Int::from_i64(&ctx, 0))));

        let x_eq = f_x.clone().add(&f_dx.clone().mul(&t));
        let x_eq_nums = Real::from_int(&Int::from_i64(&ctx, s.x as i64))
            .add(Real::from_int(&Int::from_i64(&ctx, s.delta_x as i64)).mul(&t));
        solver.assert(&Ast::_eq(&x_eq, &x_eq_nums));

        let y_eq = f_y.clone().add(&f_dy.clone().mul(&t));
        let y_eq_nums = Real::from_int(&Int::from_i64(&ctx, s.y as i64))
            .add(Real::from_int(&Int::from_i64(&ctx, s.delta_y as i64)).mul(&t));
        solver.assert(&Ast::_eq(&y_eq, &y_eq_nums));

        let z_eq = f_z.clone().add(&f_dz.clone().mul(&t));
        let z_eq_nums = Real::from_int(&Int::from_i64(&ctx, s.z as i64))
            .add(Real::from_int(&Int::from_i64(&ctx, s.delta_z as i64)).mul(&t));
        solver.assert(&Ast::_eq(&z_eq, &z_eq_nums));
    }

    if solver.check() != SatResult::Sat {
        println!("FAILED TO FIND ANSWER! Got result {:?}", solver.check());
    }

    let resp = solver
        .get_model()
        .unwrap()
        .eval(&(f_x + f_y + f_z), true)
        .unwrap();

    resp.to_string()
}
