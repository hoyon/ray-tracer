use crate::{Tuple, Ray};

static mut NEXT_ID: u32 = 0;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: u32
}

impl Sphere {
    pub fn new() -> Self {
        let id = unsafe { NEXT_ID };
        unsafe {
            NEXT_ID += 1;
        }
        Sphere{id}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
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

pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Intersection {t, object}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
