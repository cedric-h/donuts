use macroquad::prelude::{vec2, Vec2};

pub fn vec_to_angle(dir: Vec2) -> f32 {
    dir.y().atan2(dir.x())
}
pub fn angle_to_vec(angle: f32) -> Vec2 {
    let (y, x) = angle.sin_cos();
    vec2(x, y)
}

#[test]
fn angle_to_vec_to_angle() {
    macro_rules! assert_f32_eq {
        ( $o:expr, $w:expr) => {
            assert_f32_eq!($o, $w, "")
        };
        ( $l:expr, $r:expr, $($rest:tt)*) => {
            let l = $l;
            let r = $r;
            assert!(l - r <= f32::EPSILON, "left: {}, right: {} {}", l, r, $($rest)*)
        };
    }
    use std::f32::consts::PI;
    fn test_angle(angle: f32) {
        dbg!(angle_to_vec(angle));
        assert_f32_eq!(vec_to_angle(angle_to_vec(angle)), angle);
    }

    test_angle(PI/4.0);
    test_angle(-PI/4.0);
    test_angle(PI/2.0);
}

#[test]
fn vec_to_angle_to_vec() {
    let l = Vec2::one().normalize();
    let angle = vec_to_angle(l);
    assert_eq!(angle_to_vec(vec_to_angle(l)), l);

    assert_eq!(vec_to_angle(angle_to_vec(vec_to_angle(l))), angle);
}

pub fn slerp(q1: Vec2, q0: Vec2, t: f32) -> Vec2 {
    const MU: f32 = 1.85298109240830;
    const U: [f32; 8] = [
        1.0 / (1.0 * 3.0),
        1.0 / (2.0 * 5.0),
        1.0 / (3.0 * 7.0),
        1.0 / (4.0 * 9.0),
        1.0 / (5.0 * 11.0),
        1.0 / (6.0 * 13.0),
        1.0 / (7.0 * 15.0),
        MU / (8.0 * 17.0),
    ];
    const V: [f32; 8] = [
        1.0 / 3.0,
        2.0 / 5.0,
        3.0 / 7.0,
        4.0 / 9.0,
        5.0 / 11.0,
        6.0 / 13.0,
        7.0 / 15.0,
        MU * 8.0 / 17.0,
    ];

    let xm1 = q0.dot(q1) - 1.0;
    let d = 1.0 - t;
    let t_pow2 = t * t;
    let d_pow2 = d * d;

    let mut ts = [0.0; 8];
    let mut ds = [0.0; 8];
    for i in (0..7).rev() {
        ts[i] = (U[i] * t_pow2 - V[i]) * xm1;
        ds[i] = (U[i] * d_pow2 - V[i]) * xm1;
    }

    let f0 = t
        * (1.0
            + ts[0]
                * (1.0
                    + ts[1]
                        * (1.0
                            + ts[2]
                                * (1.0
                                    + ts[3]
                                        * (1.0
                                            + ts[4]
                                                * (1.0
                                                    + ts[5]
                                                        * (1.0 + ts[6] * (1.0 + ts[7]))))))));

    let f1 = d
        * (1.0
            + ds[0]
                * (1.0
                    + ds[1]
                        * (1.0
                            + ds[2]
                                * (1.0
                                    + ds[3]
                                        * (1.0
                                            + ds[4]
                                                * (1.0
                                                    + ds[5]
                                                        * (1.0 + ds[6] * (1.0 + ds[7]))))))));

    q0 * f0 + q1 * f1
}
