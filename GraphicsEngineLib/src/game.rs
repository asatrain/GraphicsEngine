use crate::math::{Mesh, Triangle, Vec3, Vec4};

pub struct Scene {
    pub meshes: [Mesh; 1],
}

// static SCENE: Scene = Scene {
//     meshes: &[Mesh {
//         triangles: &[
//             Triangle::new(Vec3::new(0.2, 0.2, 0.0),
//                           Vec3::new(0.2, 0.4, 0.0),
//                           Vec3::new(0.4, 0.2, 0.0)),
//             Triangle::new(Vec3::new(0.2, 0.6, 0.0),
//                           Vec3::new(0.2, 0.8, 0.0),
//                           Vec3::new(0.4, 0.8, 0.0)),
//             Triangle::new(Vec3::new(0.6, 0.2, 0.0),
//                           Vec3::new(0.8, 0.4, 0.0),
//                           Vec3::new(0.8, 0.2, 0.0)),
//             Triangle::new(Vec3::new(0.6, 0.8, 0.0),
//                           Vec3::new(0.8, 0.8, 0.0),
//                           Vec3::new(0.8, 0.6, 0.0))
// 
//         ]
//     }]
// };

pub fn update_scene(delta_time: f32) -> Scene {
    let mut scene = Scene {
        meshes: [Mesh {
            triangles: [
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
        tr.p1.x -= 1.75;
        tr.p2.x -= 1.75;
        tr.p3.x -= 1.75;
    }
    scene
}