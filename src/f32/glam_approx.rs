use crate::f32::{Mat4, Vec2, Vec3, Vec4};
use approx::{AbsDiffEq, UlpsEq};

impl AbsDiffEq for Vec2 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl UlpsEq for Vec2 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl UlpsEq for Vec4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Mat4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.abs_diff_eq(m2, epsilon)
    }
}

impl UlpsEq for Mat4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.ulps_eq(m2, epsilon, max_ulps)
    }
}
