use crate::vector::Vector;

pub fn reflect(subject_pos: Vector, subject_dir: Vector, object_pos: Vector, object_dir: Vector ) -> Vector {
    let n = subject_pos.sub(&object_pos).norm();
    let a1 = subject_dir.dot(&n);
    let a2 = object_dir.dot(&n);
    let optimized_p = (2.0 * (a1 - a2)) / 2.0;
    let new_direction = subject_dir.sub(&n.mul(optimized_p).mul(1.0));  // TODO: magic number
    return new_direction;
}

// Implementation adapted from:
// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
// TODO: Return enum not int
pub fn orientation(p: Vector, q: Vector, r: Vector) -> i32 {

    let val = (q.y - p.y) * (r.x - q.x) - (q.x- p.x) * (r.y - q.y);
    if val == 0.0 {
        return 0;
    }
    if val > 0.0 {
        return 1;
    }
    return 2;
}

pub fn on_segment(p: Vector, q: Vector, r: Vector) -> bool {
    if q.x <= f32::max(p.x, r.y) && q.x >= f32::min(p.x, r.x) && q.y <= f32::max(p.y, r.y) && q.y >= f32::min(p.y, r.y) {
        return true;
    }
    return false;
}

pub fn lines_intersect(p1: Vector, q1: Vector, p2: Vector, q2: Vector) -> bool {

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and q2 are colinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    return false;
}

pub fn euc_distance(p1: &Vector, p2: &Vector) -> f32 {
    (((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) as f32).sqrt()
}

pub fn rotate_point(p1: &Vector, origin: &Vector, amount_rads: f32) -> Vector {
    let degs = amount_rads * (180.0 / std::f32::consts::PI);
    println!("Rotating by {:?} degrees", degs);
    // p'x = cos(theta) * (px-ox) - sin(theta) * (py-oy) + ox
    let px_rot = degs.cos() * (p1.x - origin.x) - degs.sin() * (p1.y - origin.y) + origin.x;
    // p'y = sin(theta) * (px-ox) + cos(theta) * (py-oy) + oy
    let py_rot = degs.sin() * (p1.x - origin.x) + degs.cos() * (p1.y - origin.y) + origin.y;
    let ret = Vector{x: px_rot, y: py_rot};
    println!("Rotated vector: {:?}", ret);
    return ret;
}


// TODO: We need to include the object (boat perimeter segment) velocity somehow
/*
fn reflect_line(subject_pos: Vector, subject_dir: Vector, object_p1: Vector, object_p2: Vector) -> Vector {

    // Find the normals of the line segment we're reflecting off of
    let dx = object_p2.x - object_p1.x;
    let dy = object_p2.y - object_p2.y;
    let normal_1 = Vector{x: -1.0 * dy, y: dx}.norm();
    let normal_2 = Vector{x: dy, y: -1.0 * dx}.norm();

    let a = subject_dir.norm();
    let proj = normal_1.dot(&a);



    // TODO: Need a normal for the line segment
    // TODO: otherwise, how would we know what direction the berg is coming from
    return Vector{x:0.0, y:0.0};
}
*/
