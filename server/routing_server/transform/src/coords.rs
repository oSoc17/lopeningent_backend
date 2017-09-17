use newtypes::{Km};
use na::Vector3;
use std::f64::consts::PI;
use newtypes::ToF64;

pub struct Coordinate<T>(pub T, pub T);

impl<T: Clone> Clone for Coordinate<T> {
    fn clone(&self) -> Coordinate<T> {
        Coordinate(self.0.clone(), self.1.clone())
    }
}

impl<T: Into<Km>> Into<(Km, Km)> for Coordinate<T> {
    fn into(self) -> (Km, Km) {
        (self.0.into(), self.1.into())
    }
}

impl<T: Into<Km>> Into<Vector3<f64>> for Coordinate<T> {
    fn into(self) -> Vector3<f64> {
        Vector3::new(self.0.into().to_f64(), self.1.into().to_f64(), 0.0)
    }
}

pub struct Projector {
    up: Vector3<f64>,
    perp: Vector3<f64>,
    radius: Km,
}

pub fn average<'a, I>(vec_iter: I) -> Vector3<f64>
    where I: Iterator<Item = &'a Vector3<f64>>
{
    vec_iter
        .fold(Vector3::new(0., 0., 0.), |x, y| x + y)
        .normalize()
}

impl Projector {
    pub fn new(ray: Vector3<f64>, up: Vector3<f64>, radius: Km) -> Projector {
        let up = up - ray * ray.dot(&up) / (ray.dot(&ray) * up.dot(&up)).sqrt();
        Projector {
            up: up.normalize(),
            perp: up.cross(&ray).normalize(),
            radius: radius,
        }
    }
    pub fn map(&self, v: &Vector3<f64>) -> Coordinate<Km> {
        Coordinate(self.radius * v.dot(&self.perp),
                   self.radius * v.dot(&self.up))
    }
}
