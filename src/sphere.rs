use crate::{Matrix, Tuple, Ray};
use std::cell::Cell;

thread_local! {
    static NEXT_ID_COUNTER: Cell<u32> = Cell::new(0);
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: u32,
    pub transform: Matrix
}

impl Sphere {
    pub fn new() -> Self {
        let id = NEXT_ID_COUNTER.with(|next_id| {
            let next = next_id.get();
            next_id.set(next + 1);
            next
        });
        let transform = Matrix::identity();
        Sphere{id, transform}
    }

    pub fn intersect(&self, orig_ray: &Ray) -> Vec<Intersection> {
        let ray = orig_ray.transform(self.transform.invert());

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = Tuple::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Tuple::dot(&ray.direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            vec!()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec!(Intersection::new(t1, &self), Intersection::new(t2, &self))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Intersection {t, object}
    }
}

pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    intersections.iter()
                 .filter(|i| i.t >= 0.0)
                 .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_new_matrix() {
        let sphere = Sphere::new();
        assert_eq!(sphere.transform, Matrix::identity());
    }

    #[test]
    fn a_ray_intersects_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);
    }

    #[test]
    fn intersecting_at_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn ray_missing_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_whose_origin_is_within_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_correct_object() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let other_sphere = Sphere::new();

        let intersections = s.intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
        assert_ne!(intersections[1].object, &other_sphere);
    }

    #[test]
    fn hit_when_all_intersections_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec!(i1, i2);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), Intersection::new(1.0, &s));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec!(i1, i2);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), Intersection::new(1.0, &s));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec!(i1, i2);
        let i = hit(&xs);
        assert_eq!(i, None);
    }

    #[test]
    fn hit_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-1.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec!(i1, i2, i3, i4);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), Intersection::new(2.0, &s));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }
}
