use crate::math::{Mesh, Triangle, Vec3};

pub struct Game {
    pub meshes: &'static [Mesh],
}

static GAME: Game = Game {
    meshes: &[Mesh {
        triangles: &[Triangle::new(Vec3::new(0.2f32, 0.2f32, 0f32),
                                   Vec3::new(0.2f32, 0.5f32, 0f32),
                                   Vec3::new(0.5f32, 0.5f32, 0f32))]
    }]
};

pub fn update_game(delta_time: f32) -> &'static Game {
    &GAME
}