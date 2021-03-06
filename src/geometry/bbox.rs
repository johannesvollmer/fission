use std::ops::{Add, BitAnd, BitOr, Deref, Div, Mul, Sub};

use super::*;
use crate::op;
use crate::shape::*;

#[derive(Clone, Copy)]
pub struct BBox(pub A3<B>);

impl Zero for BBox {
    const ZERO: Self = BBox(A3(B::ZERO, B::ZERO, B::ZERO));
}

impl BBox {
    pub fn center(&self) -> P { P(self.map(B::center)) }
    pub fn extents(&self) -> F3 { self.map(B::extent) }

    pub fn max_extent(&self) -> (F, Dim) {
        self.extents().zip(A3(X, Y, Z), tup).fold((F::NEG_INF, X), tup_cmp_gt)
    }
}

impl Intersectable for BBox {
    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        !((*self - ray.o) / ray.d).fold(ray.range(), BitAnd::bitand).degen()
    }

    fn bbox(&self) -> BBox { unreachable!() }
    fn intersect(&self, _: R) -> Option<Its> { unreachable!() }
    fn hit_info(&self, _: Its) -> Its { unreachable!() }
    fn sample_surface(&self, _: F2) -> Its { unreachable!() }

    fn surface_area(&self) -> F {
        let A3(xe, ye, ze) = self.extents();
        2. * (xe * ye + xe * ze + ye * ze)
    }

    fn intersection_cost(&self) -> F { 1. }
}

op!(Add::add, *BBox -> *P -> BBox);
op!(Sub::sub, *BBox -> *P -> BBox);
op!(Mul::mul, *BBox -> *V -> BBox);
op!(Div::div, *BBox -> *V -> BBox);

op!(BitAnd::bitand, *BBox -> *BBox -> BBox);
op!(BitOr::bitor, *BBox -> *BBox -> BBox);
op!(BitOr::bitor, *BBox ->    *P -> BBox);

op!(Mul::mul, T -> *BBox -> BBox);
op!(Div::div, T -> *BBox -> BBox);

impl Deref for BBox {
    type Target = A3<B>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
