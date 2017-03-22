#![allow(dead_code)]
use x86::avx::*;
use super::super::*;
use sixty_four::*;


extern "platform-intrinsic" {
    fn x86_mm_fmadd_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fmadd_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fmadd_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fmadd_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    fn x86_mm_fmsub_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fmsub_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fmsub_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fmsub_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    fn x86_mm_fmaddsub_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fmaddsub_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fmaddsub_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fmaddsub_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    fn x86_mm_fmsubadd_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fmsubadd_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fmsubadd_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fmsubadd_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    fn x86_mm_fnmadd_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fnmadd_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fnmadd_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fnmadd_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    fn x86_mm_fnmsub_ps(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn x86_mm_fnmsub_pd(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    fn x86_mm256_fnmsub_ps(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    fn x86_mm256_fnmsub_pd(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
}

impl f64x2 {
    #[inline]
    pub fn fma(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmadd_pd(self, other, other2) }
    }
}

impl f64x4 {
    #[inline]
    pub fn fma(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmadd_pd(self, other, other2) }
    }
}

impl f32x4 {
    #[inline]
    pub fn fma(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmadd_ps(self, other, other2) }
    }
}

impl f32x8 {
    #[inline]
    pub fn fma(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmadd_ps(self, other, other2) }
    }
}

pub trait Fma {
    fn fmsub(self, other: Self, other2: Self) -> Self;
    fn fmaddsub(self, other: Self, other2: Self) -> Self;
    fn fmsubadd(self, other: Self, other2: Self) -> Self;
    fn fnmadd(self, other: Self, other2: Self) -> Self;
    fn fnmsub(self, other: Self, other2: Self) -> Self;
}

impl Fma for f32x8 {
    #[inline]
    fn fmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmsub_ps(self, other, other2) }
    }

    #[inline]
    fn fmaddsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmaddsub_ps(self, other, other2) }
    }

    #[inline]
    fn fmsubadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmsubadd_ps(self, other, other2) }
    }

    #[inline]
    fn fnmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fnmsub_ps(self, other, other2) }
    }

    #[inline]
    fn fnmadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fnmadd_ps(self, other, other2) }
    }
}

impl Fma for f32x4 {
    #[inline]
    fn fmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmsub_ps(self, other, other2) }
    }

    #[inline]
    fn fmaddsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmaddsub_ps(self, other, other2) }
    }

    #[inline]
    fn fmsubadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmsubadd_ps(self, other, other2) }
    }

    #[inline]
    fn fnmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fnmsub_ps(self, other, other2) }
    }

    #[inline]
    fn fnmadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fnmadd_ps(self, other, other2) }
    }
}

impl Fma for f64x4 {
    #[inline]
    fn fmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmsub_pd(self, other, other2) }
    }

    #[inline]
    fn fmaddsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmaddsub_pd(self, other, other2) }
    }

    #[inline]
    fn fmsubadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fmsubadd_pd(self, other, other2) }
    }

    #[inline]
    fn fnmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fnmsub_pd(self, other, other2) }
    }

    #[inline]
    fn fnmadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm256_fnmadd_pd(self, other, other2) }
    }
}

impl Fma for f64x2 {
    #[inline]
    fn fmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmsub_pd(self, other, other2) }
    }

    #[inline]
    fn fmaddsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmaddsub_pd(self, other, other2) }
    }

    #[inline]
    fn fmsubadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fmsubadd_pd(self, other, other2) }
    }

    #[inline]
    fn fnmsub(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fnmsub_pd(self, other, other2) }
    }

    #[inline]
    fn fnmadd(self, other: Self, other2: Self) -> Self {
        unsafe { x86_mm_fnmadd_pd(self, other, other2) }
    }
}
