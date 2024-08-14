use crate::math::{Mesh, Triangle, Vec4};

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

pub fn update_scene(scene: &mut Scene, delta_time: f32) {
    let offset = -0.005 * delta_time;
    for tr in scene.meshes[0].triangles.iter_mut() {
        tr.p1.x += offset;
        tr.p2.x += offset;
        tr.p3.x += offset;
    }
}