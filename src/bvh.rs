use std::{any::Any, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, SharedHittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    utils::random_int_range,
};

pub struct BvhNode {
    left: SharedHittable,
    right: SharedHittable,
    aabb: Aabb,
}

impl BvhNode {
    fn new_span(list: &mut Vec<SharedHittable>, start: usize, end: usize) -> Self {
        let span = end - start;

        if span == 1 {
            let left = list[start].clone();
            let right = list[start].clone();
            return Self {
                aabb: Aabb::new_surround(&left.bounding_box(), &right.bounding_box()),
                left: left,
                right: right,
            };
        } else if span == 2 {
            let left = list[start].clone();
            let right = list[start + 1].clone();
            return Self {
                aabb: Aabb::new_surround(&left.bounding_box(), &right.bounding_box()),
                left: left,
                right: right,
            };
        } else {
            let mut aabb = Aabb::empty();
            list[start..end]
                .iter()
                .for_each(|h| aabb.extend(&h.bounding_box()));

            let axis = aabb.longest_axis();
            list[start..end].sort_by(|a, b| {
                let a_box = a.bounding_box();
                let b_box = b.bounding_box();
                let a_axis_interval = a_box.axis_interval(axis);
                let b_axis_interval = b_box.axis_interval(axis);
                a_axis_interval
                    .min
                    .partial_cmp(&b_axis_interval.min)
                    .unwrap()
            });

            let mid = start + span / 2;
            let left = Self::new_span(list, start, mid);
            let right = Self::new_span(list, mid, end);

            return Self {
                aabb: aabb,
                left: Arc::new(left),
                right: Arc::new(right),
            };
        }
    }
    pub fn new(hittable: &mut HittableList) -> Self {
        let num_objects = hittable.objects.len();
        Self::new_span(&mut hittable.objects, 0, num_objects)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
        if !self.aabb.hit(ray, ray_t) {
            return HitRecord::new_miss();
        }

        let hit_left = self.left.hit(ray, ray_t);
        let hit_right = self.right.hit(
            ray,
            Interval::new(ray_t.min, if hit_left.hit { hit_left.t } else { ray_t.max }),
        );

        // Right takes precedence even of both are hits (not sure why...)
        if hit_right.hit {
            hit_right
        } else if hit_left.hit {
            hit_left
        } else {
            HitRecord::new_miss()
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
