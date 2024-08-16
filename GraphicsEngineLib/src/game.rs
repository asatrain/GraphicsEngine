use crate::math::{Mat4x4, Mesh, Triangle, Vec4};
use crate::UserInput;

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut scene = Scene {
            meshes: vec![Mesh {
                triangles: vec![
                    // front
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(-0.5, 0.5, 1.5),
                                  Vec4::new3d(0.5, 0.5, 1.5)),
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(0.5, 0.5, 1.5),
                                  Vec4::new3d(0.5, -0.5, 1.5)),
                    // back
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 2.5),
                                  Vec4::new3d(-0.5, 0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 2.5)),
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 2.5),
                                  Vec4::new3d(0.5, -0.5, 2.5)),
                    // left
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(-0.5, -0.5, 2.5),
                                  Vec4::new3d(-0.5, 0.5, 2.5)),
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(-0.5, 0.5, 2.5),
                                  Vec4::new3d(-0.5, 0.5, 1.5)),
                    // right
                    Triangle::new(Vec4::new3d(0.5, -0.5, 1.5),
                                  Vec4::new3d(0.5, -0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 2.5)),
                    Triangle::new(Vec4::new3d(0.5, -0.5, 1.5),
                                  Vec4::new3d(0.5, 0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 1.5)),
                    // top
                    Triangle::new(Vec4::new3d(-0.5, 0.5, 1.5),
                                  Vec4::new3d(-0.5, 0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 2.5)),
                    Triangle::new(Vec4::new3d(-0.5, 0.5, 1.5),
                                  Vec4::new3d(0.5, 0.5, 2.5),
                                  Vec4::new3d(0.5, 0.5, 1.5)),
                    // bot
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(-0.5, -0.5, 2.5),
                                  Vec4::new3d(0.5, -0.5, 2.5)),
                    Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                  Vec4::new3d(0.5, -0.5, 2.5),
                                  Vec4::new3d(0.5, -0.5, 1.5)),
                ]
            }]
        };
        for tr in scene.meshes[0].triangles.iter_mut() {
            tr.p1.x += 1.0;
            tr.p2.x += 1.0;
            tr.p3.x += 1.0;
        }
        scene
        // Scene {
        //     meshes: [Mesh {
        //         triangles: [
        //             Triangle::new(Vec4::new3d(-0.5, 0.0, 1.5),
        //                           Vec4::new3d(0.0, 0.0, 1.5),
        //                           Vec4::new3d(0.0, -1.0, 1.5)),
        //             Triangle::new(Vec4::new3d(0.0, 0.0, 1.5),
        //                           Vec4::new3d(0.0, 1.0, 1.5),
        //                           Vec4::new3d(-0.5, 0.0, 1.5)),
        //             Triangle::new(Vec4::new3d(0.0, 0.0, 1.5),
        //                           Vec4::new3d(0.0, 1.0, 1.5),
        //                           Vec4::new3d(2.0, 0.0, 1.5)),
        //             Triangle::new(Vec4::new3d(0.0, 0.0, 1.5),
        //                           Vec4::new3d(2.0, 0.0, 1.5),
        //                           Vec4::new3d(0.0, -1.0, 1.5))
        //         ]
        //     }]
        // };
    }
}


pub fn update_scene(scene: &mut Scene, user_input: &UserInput, delta_time: f32) {
    let mut offset = &mut movement_dir(user_input);
    offset *= 0.1f32 * delta_time;
    let translation = Mat4x4::translation(&offset);
    for mut tr in scene.meshes[0].triangles.iter_mut() {
        tr *= &translation;
    }
}

fn movement_dir(user_input: &UserInput) -> Vec4 {
    let mut dir = Vec4::default();
    if user_input.w_pressed && !user_input.s_pressed {
        dir.z = 1.0;
    } else if !user_input.w_pressed && user_input.s_pressed {
        dir.z = -1.0;
    }

    if user_input.d_pressed && !user_input.a_pressed {
        dir.x = 1.0;
    } else if !user_input.d_pressed && user_input.a_pressed {
        dir.x = -1.0;
    }
    return dir;
}
