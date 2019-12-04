use super::*;


pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse {
    #[inline(always)] pub const fn new(albedo: Tex<Color>) -> Self
    { Self { albedo } }
}

impl Bxdf for Diffuse {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color
    { if Frame::ct(*wi) <= 0. || Frame::ct(*wo) <= 0. { Color::BLACK }
      else { self.albedo.eval(uv) * F::INV_PI } }

    #[inline(always)] fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F)
    { let wo = V(CosineHemisphere::warp(s, ()));
      (self.albedo.eval(uv), wo, self.pdf(wi, wo)) }

    #[inline(always)] fn pdf(&self, _: V, wo: V) -> F
    { CosineHemisphere::pdf(*wo, ()) }
}

impl Zero for Diffuse { const ZERO: Self = Self::new(Tex::ZERO); }
