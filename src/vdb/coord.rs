use cgmath::Vector3;

pub type Coord = Vector3<i32>;

pub struct CoordUtils;

impl CoordUtils {
    pub fn hash(c: Coord) -> i64 {
        let h1 : i64 = ((1i64 << 20) - 1);
        let h2 : i64 = (c.x as i64 * 73856093i64 ^ c.y as i64 * 19349669i64 ^ c.z as i64 * 83492791i64);
        h1 & h2
    }
}