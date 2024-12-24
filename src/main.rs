use std::f32::consts::TAU;
use tiny_skia::{Paint, PathBuilder, Stroke, LineCap, Pixmap, Transform};
use nalgebra::{Vector2, Vector3, Rotation2};
type Vec2 = Vector2<f32>;
type Vec3 = Vector3<f32>;
type Rot2 = Rotation2<f32>;

fn paint_path(path: PathBuilder, pixmap: &mut Pixmap, scale: f32, width: f32) {
    let color = Vec3::new(255.0, 255.0, 255.0).lerp(&Vec3::new(0.0, 80.0, 0.0), scale*scale);
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.x as u8, color.y as u8, color.z as u8, 255);
    let stroke = Stroke { width, line_cap: LineCap::Round, ..Stroke::default()};
    pixmap.stroke_path(&path.finish().unwrap(), &paint, &stroke, Transform::identity(), None);
}

fn draw_branch(pm: &mut Pixmap, start: Vec2, dir: Vec2, skew: &Rot2, angle: &Rot2, n: u32) {
    if n > 0 {
        let scale = (n as f32) * 1.0/15.0;
        let new_dir = *skew * dir * scale;

        // Grow a sub branch from the current branch
        let sub_branch_dir = *angle * new_dir * 0.65; // 0.6 is the scale of the sub branch
        draw_branch(pm, start, sub_branch_dir, skew, angle, n-1);

        // Grow the current branch onwards
        let new_start = dir * 0.5 + start; // 0.5 is the distance between sub branches
        let mut pb = PathBuilder::new();
        pb.move_to(start.x, start.y);
        pb.line_to(new_start.x, new_start.y); // stem
        draw_branch(pm, new_start, new_dir, skew, angle, n-1);
        paint_path(pb, pm, scale*0.9, 3.0 / (1.0-scale + 0.2));
    }
}

fn main() {
    let mut pm = Pixmap::new(800, 1400).unwrap();
    let (mut pos, mut growth) = (Vec2::new(400.0, 1100.0), Vec2::new(0.0, -100.0));

    // Every iteration grows one layer of branches from bottom to top.
    for scale in (0..10).map(|i| 1.0 - 0.1 * i as f32) {
        // Draw trunk
        let mut pb = PathBuilder::new();
        pb.move_to(pos.x, pos.y);
        pos = pos + growth;
        pb.line_to(pos.x, pos.y);

        // Draw left and right branches
        let dir = |sign| Rot2::new(sign * TAU / (4.0 - scale)) * growth * scale;
        let skew = |sign| Rot2::new(sign * TAU / 100.0);
        let angle = |sign| Rot2::new(sign * TAU / (3.0 + scale * 10.0));
        if scale < 0.95 && scale > 0.15 {  // Do not draw branches to the top of the tree
            draw_branch(&mut pm, pos, dir( 1.0) * 1.5, &skew( 1.0), &angle( 1.0), 15);
            draw_branch(&mut pm, pos, dir(-1.0) * 1.5, &skew(-1.0), &angle(-1.0), 15);
        }
        growth = skew(1.0-scale) * growth;
        paint_path(pb, &mut pm, 0.6 + scale*0.4, 4.0 + scale*30.0);
    }
    pm.save_png("tree.png").unwrap();
}
