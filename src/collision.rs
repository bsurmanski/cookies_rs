use nalgebra::Vector3;

fn box_dim_overlap(apos: f32, adim: f32, bpos: f32, bdim: f32) -> bool {
    f32::abs(apos - bpos) < (adim + bdim)
}

pub struct Box3 {
    pos: Vector3<f32>, // center pos
    rad: Vector3<f32>, // half width
}

impl Box3 {
    pub fn new(p: Vector3<f32>, d: Vector3<f32>) -> Box3 {
        Box3 { pos: p, rad: d }
    }

    pub fn ground_area(&self) -> f32 {
        return (self.rad.x * 2.0) * (self.rad.z * 2.0);
    }

    pub fn collides(&self, o: &Box3) -> bool {
        for i in 0..3 {
            if !box_dim_overlap(
                self.pos[i], self.rad[i],
                o.pos[i], o.rad[i]) {
                return false;
            }
        }
        true
    }
}
