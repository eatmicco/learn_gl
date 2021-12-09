use glm::pi;

pub fn deg2rad(deg: f32) -> f32 {
    deg * (pi::<f32>() / 180.)
}