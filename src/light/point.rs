use super::*;


pub struct Point {
    intensity: Color,
    pos: P,
}

impl Point {
    #[inline(always)] pub fn new(power: Color, pos: P) -> Self
    { Self { intensity: power * F::INV_4PI, pos } }
}

impl Lighting for Point {
    #[inline(always)] fn eval(&self, pos: P) -> LightQueryResult {
        let sray = R::p2(pos, self.pos);
        (self.intensity / sray.t.sq(), sray)
    }

    #[inline(always)] fn sample(&self, pos: P, _: F2) -> LightQueryResult
    { self.eval(pos) }
}