use crate::math::{Mat3x3, Mesh, Triangle, Vec3, Vec4};
use crate::render::Camera;
use crate::UserInput;

pub struct GameObject {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<GameObject>,
}

impl Scene {
    pub fn new() -> Scene {
        let scene = Scene {
            camera: Camera {
                vertical_fov: 60.0,
                z_near: 0.1,
                z_far: 3.0,
                position: Vec3::new(0.0, 0.0, -1.0),
                rotation: Vec3::new(0.0, 0.0, 0.0),
            },
            objects: vec![GameObject {
                mesh: Mesh {
                    triangles: vec![
                        // front
                        Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                      Vec4::new3d(-0.5, 0.5, 1.5),
                                      Vec4::new3d(0.5, 0.5, 1.5)),
                        Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                                      Vec4::new3d(0.5, 0.5, 1.5),
                                      Vec4::new3d(0.5, -0.5, 1.5)),
                        //     // back
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
                },
                position: Vec3::new(0.0, 0.0, -1.0),
                rotation: Vec3::new(0.0, 0.0, 0.0),
            }],
        };
        scene
    }
}

pub fn update_scene(scene: &mut Scene, user_input: &UserInput, delta_time: f32) {
    update_camera(scene, user_input, delta_time);
    update_object(scene, delta_time);
}

fn update_object(scene: &mut Scene, delta_time: f32) {
    // let mut offset = Vec3::new(0.0, 0.0, 1.0);
    // offset *= 0.5 * delta_time;
    // let mut pos = &mut scene.objects[0].position;
    // pos += &offset;

    let rotation_offset = 45.0 * delta_time;
    let mut rotation = &mut scene.objects[0].rotation;
    rotation += &Vec3::new(0.0, 0.0, rotation_offset);
}

fn update_camera(scene: &mut Scene, user_input: &UserInput, delta_time: f32) {
    let mut camera_pos = &mut scene.camera.position;
    let mut camera_rotation = &mut scene.camera.rotation;

    let mut camera_offset = camera_movement_dir(user_input);
    camera_offset *= &Mat3x3::rotation(camera_rotation);
    camera_offset *= 0.5 * delta_time;
    camera_pos += &camera_offset;

    let mut camera_rotation_offset = camera_rotation_dir(user_input);
    camera_rotation_offset *= 15.0 * delta_time;
    camera_rotation += &camera_rotation_offset;
}

fn camera_movement_dir(user_input: &UserInput) -> Vec3 {
    let mut dir = Vec3::default();
    if user_input.w_pressed && !user_input.s_pressed {
        dir.z = 1.0;
    } else if !user_input.w_pressed && user_input.s_pressed {
        dir.z = -1.0;
    }

    if user_input.q_pressed && !user_input.e_pressed {
        dir.y = -1.0;
    } else if !user_input.q_pressed && user_input.e_pressed {
        dir.y = 1.0;
    }

    if user_input.shift_pressed {
        dir *= 2.5;
    }
    return dir;
}

fn camera_rotation_dir(user_input: &UserInput) -> Vec3 {
    let mut dir = Vec3::default();
    if user_input.d_pressed && !user_input.a_pressed {
        dir.y = 1.0;
    } else if !user_input.d_pressed && user_input.a_pressed {
        dir.y = -1.0;
    }

    if user_input.shift_pressed {
        dir *= 5.0;
    }
    return dir;
}
