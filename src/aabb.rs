use crate::{interval::Interval, point::Point, ray::Ray};

#[derive(Default, Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn new_pt(a: Point, b: Point) -> Self {
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn new_surround(boxes: impl IntoIterator<Item = Aabb>) -> Self {
        let mut iter = boxes.into_iter().peekable();
        if iter.peek().is_none() {
            return Aabb::default();
        }

        let mut result = iter.next().unwrap();

        for b in iter {
            result.x.extend(&b.x);
            result.y.extend(&b.y);
            result.z.extend(&b.z);
        }

        result
    }

    pub fn extend(&mut self, other: &Aabb) {
        self.x.extend(&other.x);
        self.y.extend(&other.y);
        self.z.extend(&other.z);
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Cannot get aabb axis interval from index {n}"),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        for axis in 0usize..3usize {
            let ax = self.axis_interval(axis);
            let inv_direction = 1.0 / ray_direction[axis];

            let t0 = (ax.min - ray_origin[axis]) * inv_direction;
            let t1 = (ax.max - ray_origin[axis]) * inv_direction;

            if t0 < t1 {
                ray_t.min.max(t0);
                ray_t.max.min(t1);
            } else {
                ray_t.min.max(t1);
                ray_t.max.min(t0);
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
