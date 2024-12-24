use std::{cell::LazyCell, f32::consts::TAU};

use tiny_skia::{Paint, PathBuilder, Stroke, LineCap, Pixmap, Transform};
use nalgebra::{Vector2 as Vec2, Rotation2};


// Based on https://fiddle.skia.org/c/@compose_path

// fn draw_star() {
//     let mut paint = Paint::default();
//     paint.set_color_rgba8(0, 127, 0, 200);
//     paint.anti_alias = true;
//
//     let path = {
//         let mut pb = PathBuilder::new();
//         const RADIUS: f32 = 250.0;
//         const CENTER: f32 = 250.0;
//         pb.move_to(CENTER + RADIUS, CENTER);
//         for i in 1..8 {
//             let a = 2.6927937 * i as f32;
//             pb.line_to(CENTER + RADIUS * a.cos(), CENTER + RADIUS * a.sin());
//         }
//         pb.finish().unwrap()
//     };
//
//     let mut stroke = Stroke::default();
//     stroke.width = 6.0;
//     stroke.line_cap = LineCap::Round;
//
//     let mut pixmap = Pixmap::new(500, 500).unwrap();
//     pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
//     pixmap.save_png("star.png").unwrap();
// }

//const BRANCH_STEM: f32 = 0.2;  // space between branches


fn draw_branch(
    pb: &mut PathBuilder,
    start: Vec2<f32>,
    l: Vec2<f32>,
    scew: &Rotation2<f32>,
    angle: &Rotation2<f32>,
    n: u32,
) {
    if n == 0 {
        //let end = start + l;
        //pb.line_to(end.x, end.y);
    } else {
        let stem = 0.5;
        let scale = (n as f32) * 1.0/15.0;
        let branch_scale: f32 = 0.6;

        // sub branch
        let sub_branch_l = *scew * (*angle) * l * scale * branch_scale;
        draw_branch(pb, start, sub_branch_l, scew, angle, n-1);

        // The rest of current branch
        let new_start = l * stem + start;
        let new_l = *scew * l * scale;
        pb.move_to(start.x, start.y);
        pb.line_to(new_start.x, new_start.y); // stem
        draw_branch(pb, new_start, new_l, scew, angle, n-1);
    }
}

fn main() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 127, 0, 200);
    paint.anti_alias = true;

    let path = {
        let mut pb = PathBuilder::new();
        let (mut pos, growth) = (Vec2::new(700.0, 1100.0), Vec2::new(0.0, -100.0));
        for scale in (0..10).map(|i| 1.0 - 0.095 * i as f32) {
            let branch_dir = |sign| Rotation2::new(sign * TAU / (4.0 - scale)) * growth * scale;
            let skew = |sign| Rotation2::new(sign * TAU / 100.0);
            let angle = |sign| Rotation2::new(sign * TAU/(5.0 + scale * 10.0));

            draw_branch(&mut pb, pos, branch_dir(1.0) * 1.5, &skew(1.0), &angle(1.0), 15);
            draw_branch(&mut pb, pos, branch_dir(-1.0) * 1.5, &skew(-1.0), &angle(-1.0), 15);

            pb.move_to(pos.x, pos.y);
            pos = pos + growth;
            pb.line_to(pos.x, pos.y);
        }
        pb.finish().unwrap()
    };

    let mut stroke = Stroke::default();
    stroke.width = 3.0;
    stroke.line_cap = LineCap::Round;

    let mut pixmap = Pixmap::new(1400, 1400).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.save_png("tree.png").unwrap();
}
