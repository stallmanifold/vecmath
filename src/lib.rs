#![crate_name = "vecmath"]
#![deny(missing_doc)]

//! A simple and generic library for vector math.
//!
//! Notice that row major is mathematical standard,
//! while OpenGL uses column major format.
//! This library supports both formats, prefixing functions with 'row_' or 'col_'.
//!
//! For row major affine transforms, use `Matrix2x3` (2D) and `Matrix3x4` (3D).
//! For column major affine transforms, use `Matrix3x2` (2D) and `Matrix4x3` (3D).
//!
//! If you are using `Matrix3` or `Matrix4`, 
//! then you need to pick either row or column major.
//!
//! Notice that there are two kinds of transforms: Positions and vectors.
//! The vector transforms ignores the translate component.
//! For example, `row_mat2x3_transform_pos2` transforms a position.
//! `row_mat2x3_transform_vec2` transforms a vector.

use std::num::{One};

/// A 2D vector.
pub type Vector2<T> = [T, ..2];

/// A 3D vector.
pub type Vector3<T> = [T, ..3];

/// A 4D vector.
pub type Vector4<T> = [T, ..4];

/// A 2x3 matrix.
///
/// To multiply two matrices use `row_mat2x3_mul`.
pub type Matrix2x3<T> = [[T, ..3], ..2];

/// A 3x2 matrix.
///
/// To multiply two matrices use `col_mat3x2_mul`.
pub type Matrix3x2<T> = [[T, ..2], ..3];

/// A 3x3 matrix.
///
/// To multiply two matrices use `row_mat3_mul` or `col_mat3_mul`.
pub type Matrix3<T> = [[T, ..3], ..3];

/// A 3x4 matrix.
///
/// To multiply two matrices use `row_mat3x4_mul`.
pub type Matrix3x4<T> = [[T, ..4], ..3];

/// A 4x3 matrix.
///
/// To multiply two matrices use `col_mat4x3_mul`.
///
/// This format can also store vertices of a quad.
pub type Matrix4x3<T> = [[T, ..3], ..4];

/// A 4x4 matrix.
///
/// To multiply two matrices use `row_mat4_mul` or `col_mat4_mul`.
pub type Matrix4<T> = [[T, ..4], ..4];

/// Computes row vector in matrix product by row.
#[inline(always)]
pub fn row_mat3x4_mul_row<T: Num + Copy>(
    a: Matrix3x4<T>, 
    b: Matrix3x4<T>,
    i: uint
) -> Vector4<T> {
    [
        vec4_dot_vec3(a[i], row_mat3x4_col(b, 0)),
        vec4_dot_vec3(a[i], row_mat3x4_col(b, 1)),
        vec4_dot_vec3(a[i], row_mat3x4_col(b, 2)),
        vec4_dot_pos3(a[i], row_mat3x4_col(b, 3))
    ]
}

/// Multiplies two matrices.
#[inline(always)]
pub fn row_mat3x4_mul<T: Num + Copy>(a: Matrix3x4<T>, b: Matrix3x4<T>) -> Matrix3x4<T> {
    [
        row_mat3x4_mul_row(a, b, 0),
        row_mat3x4_mul_row(a, b, 1),
        row_mat3x4_mul_row(a, b, 2)
    ]
}

/// Converts to a f32 vector.
#[inline(always)]
pub fn vec2_to_f32<T: ToPrimitive>(a: Vector2<T>) -> Option<Vector2<f32>> {
    Some([
        match a[0].to_f32() { None => return None, Some(x) => x },
        match a[1].to_f32() { None => return None, Some(x) => x } 
    ])
}

/// Converts to a f32 vector.
#[inline(always)]
pub fn vec3_to_f32<T: ToPrimitive>(a: Vector3<T>) -> Option<Vector3<f32>> {
    Some([
        match a[0].to_f32() { None => return None, Some(x) => x },
        match a[1].to_f32() { None => return None, Some(x) => x }, 
        match a[2].to_f32() { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 vector.
#[inline(always)]
pub fn vec4_to_f32<T: ToPrimitive>(a: Vector4<T>) -> Option<Vector4<f32>> {
    Some([
        match a[0].to_f32() { None => return None, Some(x) => x },
        match a[1].to_f32() { None => return None, Some(x) => x }, 
        match a[2].to_f32() { None => return None, Some(x) => x },
        match a[3].to_f32() { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat2x3_to_f32<T: ToPrimitive + Copy>(mat: Matrix2x3<T>) -> Option<Matrix2x3<f32>> {
    Some([
        match vec3_to_f32(mat[0]) { None => return None, Some(x) => x },
        match vec3_to_f32(mat[1]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3x2_to_f32<T: ToPrimitive + Copy>(mat: Matrix3x2<T>) -> Option<Matrix3x2<f32>> {
    Some([
        match vec2_to_f32(mat[0]) { None => return None, Some(x) => x },
        match vec2_to_f32(mat[1]) { None => return None, Some(x) => x },
        match vec2_to_f32(mat[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3_to_f32<T: ToPrimitive + Copy>(mat: Matrix3<T>) -> Option<Matrix3<f32>> {
    Some([
        match vec3_to_f32(mat[0]) { None => return None, Some(x) => x },
        match vec3_to_f32(mat[1]) { None => return None, Some(x) => x },
        match vec3_to_f32(mat[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3x4_to_f32<T: ToPrimitive + Copy>(m: Matrix3x4<T>) -> Option<Matrix3x4<f32>> {
    Some([
        match vec4_to_f32(m[0]) { None => return None, Some(x) => x },
        match vec4_to_f32(m[1]) { None => return None, Some(x) => x },
        match vec4_to_f32(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat4x3_to_f32<T: ToPrimitive + Copy>(m: Matrix4x3<T>) -> Option<Matrix4x3<f32>> {
    Some([
        match vec3_to_f32(m[0]) { None => return None, Some(x) => x },
        match vec3_to_f32(m[1]) { None => return None, Some(x) => x },
        match vec3_to_f32(m[2]) { None => return None, Some(x) => x },
        match vec3_to_f32(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat4_to_f32<T: ToPrimitive + Copy>(m: Matrix4<T>) -> Option<Matrix4<f32>> {
    Some([
        match vec4_to_f32(m[0]) { None => return None, Some(x) => x },
        match vec4_to_f32(m[1]) { None => return None, Some(x) => x },
        match vec4_to_f32(m[2]) { None => return None, Some(x) => x },
        match vec4_to_f32(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec2_to_f64<T: ToPrimitive>(a: Vector2<T>) -> Option<Vector2<f64>> {
    Some([
        match a[0].to_f64() { None => return None, Some(x) => x },
        match a[1].to_f64() { None => return None, Some(x) => x } 
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec3_to_f64<T: ToPrimitive>(a: Vector3<T>) -> Option<Vector3<f64>> {
    Some([
        match a[0].to_f64() { None => return None, Some(x) => x },
        match a[1].to_f64() { None => return None, Some(x) => x }, 
        match a[2].to_f64() { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec4_to_f64<T: ToPrimitive>(a: Vector4<T>) -> Option<Vector4<f64>> {
    Some([
        match a[0].to_f64() { None => return None, Some(x) => x },
        match a[1].to_f64() { None => return None, Some(x) => x }, 
        match a[2].to_f64() { None => return None, Some(x) => x },
        match a[3].to_f64() { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat2x3_to_f64<T: ToPrimitive + Copy>(m: Matrix2x3<T>) -> Option<Matrix2x3<f64>> {
    Some([
        match vec3_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[1]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3x2_to_f64<T: ToPrimitive + Copy>(m: Matrix3x2<T>) -> Option<Matrix3x2<f64>> {
    Some([
        match vec2_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec2_to_f64(m[1]) { None => return None, Some(x) => x },
        match vec2_to_f64(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3_to_f64<T: ToPrimitive + Copy>(m: Matrix3<T>) -> Option<Matrix3<f64>> {
    Some([
        match vec3_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[1]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3x4_to_f64<T: ToPrimitive + Copy>(m: Matrix3x4<T>) -> Option<Matrix3x4<f64>> {
    Some([
        match vec4_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec4_to_f64(m[1]) { None => return None, Some(x) => x },
        match vec4_to_f64(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat4x3_to_f64<T: ToPrimitive + Copy>(m: Matrix4x3<T>) -> Option<Matrix4x3<f64>> {
    Some([
        match vec3_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[1]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[2]) { None => return None, Some(x) => x },
        match vec3_to_f64(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat4_to_f64<T: ToPrimitive + Copy>(m: Matrix4<T>) -> Option<Matrix4<f64>> {
    Some([
        match vec4_to_f64(m[0]) { None => return None, Some(x) => x },
        match vec4_to_f64(m[1]) { None => return None, Some(x) => x },
        match vec4_to_f64(m[2]) { None => return None, Some(x) => x },
        match vec4_to_f64(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts from a f32 vector.
#[inline(always)]
pub fn vec2_from_f32<T: FromPrimitive>(a: Vector2<f32>) -> Option<Vector2<T>> {
    Some([
        match FromPrimitive::from_f32(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f32(a[1]) { None => return None, Some(x) => x } 
    ])
}

/// Converts from a f32 vector.
#[inline(always)]
pub fn vec3_from_f32<T: FromPrimitive>(a: Vector3<f32>) -> Option<Vector3<T>> {
    Some([
        match FromPrimitive::from_f32(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f32(a[1]) { None => return None, Some(x) => x }, 
        match FromPrimitive::from_f32(a[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 vector.
#[inline(always)]
pub fn vec4_from_f32<T: FromPrimitive>(a: Vector4<f32>) -> Option<Vector4<T>> {
    Some([
        match FromPrimitive::from_f32(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f32(a[1]) { None => return None, Some(x) => x }, 
        match FromPrimitive::from_f32(a[2]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f32(a[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat2x3_from_f32<T: FromPrimitive + Copy>(m: Matrix2x3<f32>) -> Option<Matrix2x3<T>> {
    Some([
        match vec3_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[1]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3x2_from_f32<T: FromPrimitive + Copy>(m: Matrix3x2<f32>) -> Option<Matrix3x2<T>> {
    Some([
        match vec2_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec2_from_f32(m[1]) { None => return None, Some(x) => x },
        match vec2_from_f32(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3_from_f32<T: FromPrimitive + Copy>(m: Matrix3<f32>) -> Option<Matrix3<T>> {
    Some([
        match vec3_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[1]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat3x4_from_f32<T: FromPrimitive + Copy>(m: Matrix3x4<f32>) -> Option<Matrix3x4<T>> {
    Some([
        match vec4_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec4_from_f32(m[1]) { None => return None, Some(x) => x },
        match vec4_from_f32(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 column matrix.
#[inline(always)]
pub fn mat4x3_from_f32<T: FromPrimitive + Copy>(m: Matrix4x3<f32>) -> Option<Matrix4x3<T>> {
    Some([
        match vec3_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[1]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[2]) { None => return None, Some(x) => x },
        match vec3_from_f32(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f32 matrix.
#[inline(always)]
pub fn mat4_from_f32<T: FromPrimitive + Copy>(m: Matrix4<f32>) -> Option<Matrix4<T>> {
    Some([
        match vec4_from_f32(m[0]) { None => return None, Some(x) => x },
        match vec4_from_f32(m[1]) { None => return None, Some(x) => x },
        match vec4_from_f32(m[2]) { None => return None, Some(x) => x },
        match vec4_from_f32(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec2_from_f64<T: FromPrimitive>(a: Vector2<f64>) -> Option<Vector2<T>> {
    Some([
        match FromPrimitive::from_f64(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f64(a[1]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec3_from_f64<T: FromPrimitive>(a: Vector3<f64>) -> Option<Vector3<T>> {
    Some([
        match FromPrimitive::from_f64(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f64(a[1]) { None => return None, Some(x) => x }, 
        match FromPrimitive::from_f64(a[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 vector.
#[inline(always)]
pub fn vec4_from_f64<T: FromPrimitive>(a: Vector4<f64>) -> Option<Vector4<T>> {
    Some([
        match FromPrimitive::from_f64(a[0]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f64(a[1]) { None => return None, Some(x) => x }, 
        match FromPrimitive::from_f64(a[2]) { None => return None, Some(x) => x },
        match FromPrimitive::from_f64(a[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat2x3_from_f64<T: FromPrimitive + Copy>(mat: Matrix2x3<f64>) -> Option<Matrix2x3<T>> {
    Some([
        match vec3_from_f64(mat[0]) { None => return None, Some(x) => x },
        match vec3_from_f64(mat[1]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3x2_from_f64<T: FromPrimitive + Copy>(mat: Matrix3x2<f64>) -> Option<Matrix3x2<T>> {
    Some([
        match vec2_from_f64(mat[0]) { None => return None, Some(x) => x },
        match vec2_from_f64(mat[1]) { None => return None, Some(x) => x },
        match vec2_from_f64(mat[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3_from_f64<T: FromPrimitive + Copy>(mat: Matrix3<f64>) -> Option<Matrix3<T>> {
    Some([
        match vec3_from_f64(mat[0]) { None => return None, Some(x) => x },
        match vec3_from_f64(mat[1]) { None => return None, Some(x) => x },
        match vec3_from_f64(mat[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat3x4_from_f64<T: FromPrimitive + Copy>(m: Matrix3x4<f64>) -> Option<Matrix3x4<T>> {
    Some([
        match vec4_from_f64(m[0]) { None => return None, Some(x) => x },
        match vec4_from_f64(m[1]) { None => return None, Some(x) => x },
        match vec4_from_f64(m[2]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat4x3_from_f64<T: FromPrimitive + Copy>(m: Matrix4x3<f64>) -> Option<Matrix4x3<T>> {
    Some([
        match vec3_from_f64(m[0]) { None => return None, Some(x) => x },
        match vec3_from_f64(m[1]) { None => return None, Some(x) => x },
        match vec3_from_f64(m[2]) { None => return None, Some(x) => x },
        match vec3_from_f64(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Converts to a f64 matrix.
#[inline(always)]
pub fn mat4_from_f64<T: FromPrimitive + Copy>(m: Matrix4<f64>) -> Option<Matrix4<T>> {
    Some([
        match vec4_from_f64(m[0]) { None => return None, Some(x) => x },
        match vec4_from_f64(m[1]) { None => return None, Some(x) => x },
        match vec4_from_f64(m[2]) { None => return None, Some(x) => x },
        match vec4_from_f64(m[3]) { None => return None, Some(x) => x }
    ])
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn vec2_sub<T: Num>(a: Vector2<T>, b: Vector2<T>) -> Vector2<T> {
    [
        a[0] - b[0],
        a[1] - b[1],
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn vec3_sub<T: Num>(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn vec4_sub<T: Num>(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> {
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
        a[3] - b[3]
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat2x3_sub<T: Num + Copy>(a: Matrix2x3<T>, b: Matrix2x3<T>) -> Matrix2x3<T> {
    [
        vec3_sub(a[0], b[0]),
        vec3_sub(a[1], b[1])
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat3x2_sub<T: Num + Copy>(a: Matrix3x2<T>, b: Matrix3x2<T>) -> Matrix3x2<T> {
    [
        vec2_sub(a[0], b[0]),
        vec2_sub(a[1], b[1]),
        vec2_sub(a[2], b[2])
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat3_sub<T: Num + Copy>(a: Matrix3<T>, b: Matrix3<T>) -> Matrix3<T> {
    [
        vec3_sub(a[0], b[0]),
        vec3_sub(a[1], b[1]),
        vec3_sub(a[2], b[2])
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat3x4_sub<T: Num + Copy>(a: Matrix3x4<T>, b: Matrix3x4<T>) -> Matrix3x4<T> {
    [
        vec4_sub(a[0], b[0]),
        vec4_sub(a[1], b[1]),
        vec4_sub(a[2], b[2])
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat4x3_sub<T: Num + Copy>(a: Matrix4x3<T>, b: Matrix4x3<T>) -> Matrix4x3<T> {
    [
        vec3_sub(a[0], b[0]),
        vec3_sub(a[1], b[1]),
        vec3_sub(a[2], b[2]),
        vec3_sub(a[3], b[3])
    ]
}

/// Subtracts 'b' from 'a'.
#[inline(always)]
pub fn mat4_sub<T: Num + Copy>(a: Matrix4<T>, b: Matrix4<T>) -> Matrix4<T> {
    [
        vec4_sub(a[0], b[0]),
        vec4_sub(a[1], b[1]),
        vec4_sub(a[2], b[2]),
        vec4_sub(a[3], b[3])
    ]
}

/// Adds two vectors.
#[inline(always)]
pub fn vec2_add<T: Num>(a: Vector2<T>, b: Vector2<T>) -> Vector2<T> {
    [
        a[0] + b[0],
        a[1] + b[1],
    ]
}

/// Adds two vectors.
#[inline(always)]
pub fn vec3_add<T: Num>(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2]
    ]
}

/// Adds two vectors.
#[inline(always)]
pub fn vec4_add<T: Num>(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3]
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat2x3_add<T: Num + Copy>(a: Matrix2x3<T>, b: Matrix2x3<T>) -> Matrix2x3<T> {
    [
        vec3_add(a[0], b[0]),
        vec3_add(a[1], b[1])
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat3x2_add<T: Num + Copy>(a: Matrix3x2<T>, b: Matrix3x2<T>) -> Matrix3x2<T> {
    [
        vec2_add(a[0], b[0]),
        vec2_add(a[1], b[1]),
        vec2_add(a[2], b[2])
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat3_add<T: Num + Copy>(a: Matrix3<T>, b: Matrix3<T>) -> Matrix3<T> {
    [
        vec3_add(a[0], b[0]),
        vec3_add(a[1], b[1]),
        vec3_add(a[2], b[2])
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat3x4_add<T: Num + Copy>(a: Matrix3x4<T>, b: Matrix3x4<T>) -> Matrix3x4<T> {
    [
        vec4_add(a[0], b[0]),
        vec4_add(a[1], b[1]),
        vec4_add(a[2], b[2])
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat4x3_add<T: Num + Copy>(a: Matrix4x3<T>, b: Matrix4x3<T>) -> Matrix4x3<T> {
    [
        vec3_add(a[0], b[0]),
        vec3_add(a[1], b[1]),
        vec3_add(a[2], b[2]),
        vec3_add(a[3], b[3])
    ]
}

/// Adds two matrices.
#[inline(always)]
pub fn mat4_add<T: Num + Copy>(a: Matrix4<T>, b: Matrix4<T>) -> Matrix4<T> {
    [
        vec4_add(a[0], b[0]),
        vec4_add(a[1], b[1]),
        vec4_add(a[2], b[2]),
        vec4_add(a[3], b[3])
    ]
}

/// Computes the dot product.
#[inline(always)]
pub fn vec2_dot<T: Num>(a: Vector2<T>, b: Vector2<T>) -> T {
    a[0] * b[0] + a[1] * b[1]
}

/// Computes the dot product.
#[inline(always)]
pub fn vec3_dot<T: Num>(a: Vector3<T>, b: Vector3<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Computes the square length of a vector.
#[inline(always)]
pub fn vec2_square_len<T: Num>(a: Vector2<T>) -> T {
    a[0] * a[0] + a[1] * a[1]
}

/// Computes the square length of a vector.
#[inline(always)]
pub fn vec3_square_len<T: Num>(a: Vector3<T>) -> T {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

/// Computes the cross product.
#[inline(always)]
pub fn vec2_cross<T: Num>(a: Vector2<T>, b: Vector2<T>) -> T {
    a[1] * b[2] - a[2] * b[1]
}

/// Computes the cross product.
#[inline(always)]
pub fn vec3_cross<T: Num>(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]
}

/// Multiplies the vector with a scalar.
#[inline(always)]
pub fn vec2_scale<T: Num>(a: Vector2<T>, b: T) -> Vector2<T> {
    [
        a[0] * b,
        a[1] * b
    ]
}

/// Multiplies the vector with a scalar.
#[inline(always)]
pub fn vec3_scale<T: Num>(a: Vector3<T>, b: T) -> Vector3<T> {
    [
        a[0] * b,
        a[1] * b,
        a[2] * b
    ]
}

/// Computes the length of vector.
#[inline(always)]
pub fn vec2_len<T: Float>(a: Vector2<T>) -> T {
    vec2_square_len(a).sqrt()
}

/// Computes the length of vector.
#[inline(always)]
pub fn vec3_len<T: Float>(a: Vector3<T>) -> T {
    vec3_square_len(a).sqrt()
}

/// Computes the inverse length of a vector.
#[inline(always)]
pub fn vec2_inv_len<T: Float>(a: Vector2<T>) -> T {
    let one: T = One::one();
    one / vec2_len(a)
}

/// Computes the inverse length of a vector.
#[inline(always)]
pub fn vec3_inv_len<T: Float>(a: Vector3<T>) -> T {
    let one: T = One::one();
    one / vec3_len(a)
}

/// Computes the normalized.
#[inline(always)]
pub fn vec2_normalized<T: Float>(a: Vector2<T>) -> Vector2<T> {
    vec2_scale(a, vec2_inv_len(a))
}

/// Computes the normalized.
#[inline(always)]
pub fn vec3_normalized<T: Float>(a: Vector3<T>) -> Vector3<T> {
    vec3_scale(a, vec3_inv_len(a))
}

/// Computes the normalized difference between two vectors.
///
/// This is often used to get direction from 'b' to 'a'.
#[inline(always)]
pub fn vec2_normalized_sub<T: Float>(
    a: Vector2<T>, 
    b: Vector2<T>
) -> Vector2<T> {
    vec2_normalized(vec2_sub(a, b))
}

/// Computes the normalized difference between two vectors.
///
/// This is often used to get direction from 'b' to 'a'.
#[inline(always)]
pub fn vec3_normalized_sub<T: Float>(
    a: Vector3<T>, 
    b: Vector3<T>
) -> Vector3<T> {
    vec3_normalized(vec3_sub(a, b))
}

/// Computes transformed vector component.
///
/// This is used when transforming vectors through matrices.
#[inline(always)]
pub fn vec3_dot_vec2<T: Num>(a: Vector3<T>, b: Vector2<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Computes transformed vector component.
///
/// This is used when transforming vectors through matrices.
#[inline(always)]
pub fn vec4_dot_vec3<T: Num>(a: Vector4<T>, b: Vector3<T>) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Computes transformed position component.
///
/// This is used when transforming points through matrices.
#[inline(always)]
pub fn vec3_dot_pos2<T: Num + Copy>(a: Vector3<T>, b: Vector2<T>) -> T {
    vec3_dot_vec2(a, b) + a[3]
}

/// Computes transformed position component.
///
/// This is used when transforming points through matrices.
#[inline(always)]
pub fn vec4_dot_pos3<T: Num + Copy>(a: Vector4<T>, b: Vector3<T>) -> T {
    vec4_dot_vec3(a, b) + a[3]
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat2x3_col<T: Copy>(mat: Matrix2x3<T>, i: uint) -> Vector2<T> {
    [mat[0][i], mat[1][i]]
}

/// Returns a row vector of a column matrix.
#[inline(always)]
pub fn col_mat2x3_row<T: Copy>(mat: Matrix2x3<T>, i: uint) -> Vector2<T> {
    row_mat2x3_col(mat, i)
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat3x2_col<T: Copy>(a: Matrix3x2<T>, i: uint) -> Vector3<T> {
    [a[0][i], a[1][i], a[2][i]]
}

/// Returns a row vector of a column matrix.
#[inline(always)]
pub fn col_mat3x2_row<T: Copy>(a: Matrix3x2<T>, i: uint) -> Vector3<T> {
    row_mat3x2_col(a, i)
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat3_col<T: Copy>(a: Matrix3<T>, i: uint) -> Vector3<T> {
    [a[0][i], a[1][i], a[2][i]]
}

/// Returns a row vector of a column matrix.
#[inline(always)]
pub fn col_mat3_row<T: Copy>(a: Matrix3<T>, i: uint) -> Vector3<T> {
    row_mat3_col(a, i)
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat3x4_col<T: Copy>(mat: Matrix3x4<T>, i: uint) -> Vector3<T> {
    [mat[0][i], mat[1][i], mat[2][i]]
}

/// Returns a row vector of a column matrix.
#[inline(always)]
pub fn col_mat3x4_row<T: Copy>(mat: Matrix3x4<T>, i: uint) -> Vector3<T> {
    row_mat3x4_col(mat, i)
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat4x3_col<T: Copy>(a: Matrix4x3<T>, i: uint) -> Vector4<T> {
    [a[0][i], a[1][i], a[2][i], a[3][i]]
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn col_mat4x3_row<T: Copy>(a: Matrix4x3<T>, i: uint) -> Vector4<T> {
    row_mat4x3_col(a, i)
}

/// Returns a column vector of a row matrix.
#[inline(always)]
pub fn row_mat4_col<T: Copy>(a: Matrix4<T>, i: uint) -> Vector4<T> {
    [a[0][i], a[1][i], a[2][i], a[3][i]]
}

/// Returns a row vector of a column matrix.
#[inline(always)]
pub fn col_mat4_row<T: Copy>(a: Matrix4<T>, i: uint) -> Vector4<T> {
    row_mat4_col(a, i)
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat2x3_transposed<T: Copy>(a: Matrix2x3<T>) -> Matrix3x2<T> {
    [
        row_mat2x3_col(a, 0),
        row_mat2x3_col(a, 1),
        row_mat2x3_col(a, 2)
    ]
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat3x2_transposed<T: Copy>(a: Matrix3x2<T>) -> Matrix2x3<T> {
    [
        row_mat3x2_col(a, 0),
        row_mat3x2_col(a, 1)
    ]
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat3_transposed<T: Copy>(a: Matrix3<T>) -> Matrix3<T> {
    [
        row_mat3_col(a, 0),
        row_mat3_col(a, 1),
        row_mat3_col(a, 2)
    ]
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat3x4_transposed<T: Copy>(a: Matrix3x4<T>) -> Matrix4x3<T> {
    [
        row_mat3x4_col(a, 0),
        row_mat3x4_col(a, 1),
        row_mat3x4_col(a, 2),
        row_mat3x4_col(a, 3)
    ]
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat4x3_transposed<T: Copy>(a: Matrix4x3<T>) -> Matrix3x4<T> {
    [
        row_mat4x3_col(a, 0),
        row_mat4x3_col(a, 1),
        row_mat4x3_col(a, 2)
    ]
}

/// Constructs the transpose of a matrix.
#[inline(always)]
pub fn mat4_transposed<T: Copy>(a: Matrix4<T>) -> Matrix4<T> {
    [
        row_mat4_col(a, 0),
        row_mat4_col(a, 1),
        row_mat4_col(a, 2),
        row_mat4_col(a, 3)
    ]
}

/// Transforms a 2D position through matrix.
#[inline(always)]
pub fn row_mat2x3_transform_pos2<T: Num + Copy>(
    mat: Matrix2x3<T>, 
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_pos2(mat[0], a),
        vec3_dot_pos2(mat[1], a)
    ]
}

/// Transforms a 2D position through matrix.
#[inline(always)]
pub fn col_mat3x2_transform_pos2<T: Num + Copy>(
    mat: Matrix3x2<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_pos2(col_mat3x2_row(mat, 0), a),
        vec3_dot_pos2(col_mat3x2_row(mat, 1), a)
    ]
}

/// Transforms a 2D position through row matrix.
#[inline(always)]
pub fn row_mat3_transform_pos2<T: Num + Copy>(
    mat: Matrix3<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_pos2(mat[0], a),
        vec3_dot_pos2(mat[1], a)
    ]
}

/// Transforms a 2D position through column matrix.
#[inline(always)]
pub fn col_mat3_transform_pos2<T: Num + Copy>(
    mat: Matrix3<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_pos2(col_mat3_row(mat, 0), a),
        vec3_dot_pos2(col_mat3_row(mat, 1), a)
    ]
}

/// Transforms a 3D position through matrix.
#[inline(always)]
pub fn row_mat3x4_transform_pos3<T: Num + Copy>(
    mat: Matrix3x4<T>, 
    a: Vector3<T>
) -> Vector3<T> {
    [
        vec4_dot_pos3(mat[0], a),
        vec4_dot_pos3(mat[1], a),
        vec4_dot_pos3(mat[2], a),
    ]
}

/// Transforms a 3D position through matrix.
#[inline(always)]
pub fn col_mat4x3_transform_pos3<T: Num + Copy>(
    mat: Matrix4x3<T>,
    a: Vector3<T>
) -> Vector3<T> {
    [
        vec4_dot_pos3(col_mat4x3_row(mat, 0), a),
        vec4_dot_pos3(col_mat4x3_row(mat, 1), a),
        vec4_dot_pos3(col_mat4x3_row(mat, 2), a)
    ]
}

/// Transforms a 2D vector through matrix.
#[inline(always)]
pub fn row_mat2x3_transform_vec2<T: Num + Copy>(
    mat: Matrix2x3<T>, 
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_vec2(mat[0], a),
        vec3_dot_vec2(mat[1], a)
    ]
}

/// Transforms a 2D vector through matrix.
#[inline(always)]
pub fn col_mat3x2_transform_vec2<T: Num + Copy>(
    mat: Matrix3x2<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_vec2(col_mat3x2_row(mat, 0), a),
        vec3_dot_vec2(col_mat3x2_row(mat, 1), a)
    ]
}

/// Transforms a 2D vector through row matrix.
#[inline(always)]
pub fn row_mat3_transform_vec2<T: Num + Copy>(
    mat: Matrix3<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_vec2(mat[0], a),
        vec3_dot_vec2(mat[1], a)
    ]
}

/// Transforms a 2D vector through column matrix.
#[inline(always)]
pub fn col_mat3_transform_vec2<T: Num + Copy>(
    mat: Matrix3<T>,
    a: Vector2<T>
) -> Vector2<T> {
    [
        vec3_dot_vec2(col_mat3_row(mat, 0), a),
        vec3_dot_vec2(col_mat3_row(mat, 1), a)
    ]
}

/// Transforms a 3D vector through matrix.
#[inline(always)]
pub fn row_mat3x4_transform_vec3<T: Num + Copy>(
    mat: Matrix3x4<T>, 
    a: Vector3<T>
) -> Vector3<T> {
    [
        vec4_dot_vec3(mat[0], a),
        vec4_dot_vec3(mat[1], a),
        vec4_dot_vec3(mat[2], a)
    ]
}

/// Transforms a 3D vector through matrix.
#[inline(always)]
pub fn col_mat4x3_transform_vec3<T: Num + Copy>(
    mat: Matrix4x3<T>,
    a: Vector3<T>
) -> Vector3<T> {
    [
        vec4_dot_vec3(col_mat4x3_row(mat, 0), a),
        vec4_dot_vec3(col_mat4x3_row(mat, 1), a),
        vec4_dot_vec3(col_mat4x3_row(mat, 2), a)
    ]
}

/// Computes the determinant of a matrix.
pub fn mat2x3_det<T: Num>(mat: Matrix2x3<T>) -> T {
      mat[0][0] * mat[1][1]
    - mat[0][1] * mat[1][0]
}

/// Computes the determinant of a matrix.
pub fn mat3x2_det<T: Num>(mat: Matrix3x2<T>) -> T {
      mat[0][0] * mat[1][1]
    - mat[0][1] * mat[1][0]
}

/// Computes the determinant of a matrix.
pub fn mat3x4_det<T: Num>(mat: Matrix3x4<T>) -> T {
      mat[0][0] * mat[1][1] * mat[2][2]
    + mat[0][1] * mat[1][2] * mat[2][0]
    + mat[0][2] * mat[1][0] * mat[2][1]
    - mat[0][0] * mat[1][2] * mat[2][1]
    - mat[0][1] * mat[1][0] * mat[2][2]
    - mat[0][2] * mat[1][1] * mat[2][0]
}

/// Computes the determinant of a matrix.
pub fn mat4x3_det<T: Num>(mat: Matrix4x3<T>) -> T {
      mat[0][0] * mat[1][1] * mat[2][2]
    + mat[0][1] * mat[1][2] * mat[2][0]
    + mat[0][2] * mat[1][0] * mat[2][1]
    - mat[0][0] * mat[1][2] * mat[2][1]
    - mat[0][1] * mat[1][0] * mat[2][2]
    - mat[0][2] * mat[1][1] * mat[2][0]
}

/// Computes the determinant of a 4x4 matrix.
pub fn mat4_det<T: Num>(mat: Matrix4<T>) -> T {
      mat[0][0] * mat[1][1] * mat[2][2] * mat[3][3] 
    + mat[0][0] * mat[1][2] * mat[2][3] * mat[3][1]
    + mat[0][0] * mat[1][3] * mat[2][1] * mat[3][2]

    + mat[0][1] * mat[1][0] * mat[2][3] * mat[3][2]
    + mat[0][1] * mat[1][2] * mat[2][0] * mat[3][3]
    + mat[0][1] * mat[1][3] * mat[2][2] * mat[3][0]

    + mat[0][2] * mat[1][0] * mat[2][1] * mat[3][3]
    + mat[0][2] * mat[1][1] * mat[2][3] * mat[3][0]
    + mat[0][2] * mat[1][3] * mat[2][0] * mat[3][1]

    + mat[0][3] * mat[1][0] * mat[2][2] * mat[3][1]
    + mat[0][3] * mat[1][1] * mat[2][0] * mat[3][2]
    + mat[0][3] * mat[1][2] * mat[2][1] * mat[3][0]

    - mat[0][0] * mat[1][1] * mat[2][3] * mat[3][2]
    - mat[0][0] * mat[1][2] * mat[2][1] * mat[3][3]
    - mat[0][0] * mat[1][3] * mat[2][2] * mat[3][1]

    - mat[0][1] * mat[1][0] * mat[2][2] * mat[3][3]
    - mat[0][1] * mat[1][2] * mat[2][3] * mat[3][0]
    - mat[0][1] * mat[1][3] * mat[2][0] * mat[3][2]

    - mat[0][2] * mat[1][0] * mat[2][3] * mat[3][1]
    - mat[0][2] * mat[1][1] * mat[2][0] * mat[3][3]
    - mat[0][2] * mat[1][3] * mat[2][1] * mat[3][0]

    - mat[0][3] * mat[1][0] * mat[2][1] * mat[3][2]
    - mat[0][3] * mat[1][1] * mat[2][2] * mat[3][0]
    - mat[0][3] * mat[1][2] * mat[2][0] * mat[3][1]
}

/// Computes inverse determinant of a 2x3 matrix.
#[inline(always)]
pub fn mat2x3_inv_det<T: Num>(mat: Matrix2x3<T>) -> T {
    let one: T = One::one();
    one / mat2x3_det(mat)
}

/// Computes inverse determinant of a 3x4 matrix.
#[inline(always)]
pub fn mat3x4_inv_det<T: Num>(mat: Matrix3x4<T>) -> T {
    let one: T = One::one();
    one / mat3x4_det(mat)
}

/// Computes the inverse determinant of a 4x4 matrix.
#[inline(always)]
pub fn mat4_inv_det<T: Num>(mat: Matrix4<T>) -> T {
    let one: T = One::one();
    one / mat4_det(mat)
}

/// Computes the inverse of a 2x3 matrix.
pub fn mat2x3_inv<T: Num + Copy>(mat: Matrix2x3<T>) -> Matrix2x3<T> {
    let inv_det = mat2x3_inv_det(mat);

    [
        [
              mat[1][1] * inv_det,
            - mat[0][1] * inv_det,
            (
              mat[0][1] * mat[1][2]
            - mat[0][2] * mat[1][1]
            ) * inv_det
        ],
        [
            - mat[1][0] * inv_det,
              mat[0][0] * inv_det,
            (
              mat[0][2] * mat[1][0]
            - mat[0][0] * mat[1][2]
            ) * inv_det,
        ]
    ]
}

/// Computes the inverse of a 3x4 matrix.
pub fn mat3x4_inv<T: Num + Copy>(mat: Matrix3x4<T>) -> Matrix3x4<T> {
    let inv_det = mat3x4_inv_det(mat);

    [
        [   (
              mat[1][1] * mat[2][2]
            - mat[1][2] * mat[2][1]
            ) * inv_det,
            (
              mat[0][2] * mat[2][1]
            - mat[0][1] * mat[2][2]
            ) * inv_det,
            (
              mat[0][1] * mat[1][2]
            - mat[0][2] * mat[1][1]
            ) * inv_det,
            (
              mat[0][1] * mat[1][3] * mat[2][2]
            + mat[0][2] * mat[1][1] * mat[2][3]
            + mat[0][3] * mat[1][2] * mat[2][1]
            - mat[0][1] * mat[1][2] * mat[2][3]
            - mat[0][2] * mat[1][3] * mat[2][1]
            - mat[0][3] * mat[1][1] * mat[2][2]
            ) * inv_det
        ],
        [
            (
              mat[1][2] * mat[2][0]
            - mat[1][0] * mat[2][2]
            ) * inv_det,
            (
              mat[0][0] * mat[2][2]
            - mat[0][2] * mat[2][0]
            ) * inv_det,
            (
              mat[0][2] * mat[1][0]
            - mat[0][0] * mat[1][2]
            ) * inv_det,
            (
              mat[0][0] * mat[1][2] * mat[2][3]
            + mat[0][2] * mat[1][3] * mat[2][0]
            + mat[0][3] * mat[1][0] * mat[2][2]
            - mat[0][0] * mat[1][3] * mat[2][2]
            - mat[0][2] * mat[1][0] * mat[2][3]
            - mat[0][3] * mat[1][2] * mat[2][0]
            ) * inv_det
        ],
        [
            (
              mat[1][0] * mat[2][1]
            - mat[1][1] * mat[2][0]
            ) * inv_det,
            (
              mat[0][1] * mat[2][0]
            - mat[0][0] * mat[2][1]
            ) * inv_det,
            (
              mat[0][0] * mat[1][1]
            - mat[0][1] * mat[1][0]
            ) * inv_det,
            (
              mat[0][0] * mat[1][3] * mat[2][1]
            + mat[0][1] * mat[1][0] * mat[2][3]
            + mat[0][3] * mat[1][1] * mat[2][0]
            - mat[0][0] * mat[1][1] * mat[2][3]
            - mat[0][1] * mat[1][3] * mat[2][0]
            - mat[0][3] * mat[1][0] * mat[2][1]
            ) * inv_det
        ]
    ]
}
/// Computes the inverse of a 4x4 matrix.
pub fn mat4_inv<T: Num + Copy>(mat: Matrix4<T>) -> Matrix4<T> {
    let inv_det = mat4_inv_det(mat);

    [
        [   (
                mat[1][1] * mat[2][2] * mat[3][3]
                + mat[1][2] * mat[2][3] * mat[3][1]
                + mat[1][3] * mat[2][1] * mat[3][2]
                - mat[1][1] * mat[2][3] * mat[3][2]
                - mat[1][2] * mat[2][1] * mat[3][3]
                - mat[1][3] * mat[2][2] * mat[3][1]
            ) * inv_det,
            (
                mat[0][1] * mat[2][3] * mat[3][2]
                + mat[0][2] * mat[2][1] * mat[3][3]
                + mat[0][3] * mat[2][2] * mat[3][1]
                - mat[0][1] * mat[2][2] * mat[3][3]
                - mat[0][2] * mat[2][3] * mat[3][1]
                - mat[0][3] * mat[2][1] * mat[3][2]
            ) * inv_det,
            (
                mat[0][1] * mat[1][2] * mat[3][3]
                + mat[0][2] * mat[1][3] * mat[3][1]
                + mat[0][3] * mat[1][1] * mat[3][2]
                - mat[0][1] * mat[1][3] * mat[3][2]
                - mat[0][2] * mat[1][1] * mat[3][3]
                - mat[0][3] * mat[1][2] * mat[3][1]
            ) * inv_det,
            (
                mat[0][1] * mat[1][3] * mat[2][2]
                + mat[0][2] * mat[1][1] * mat[2][3]
                + mat[0][3] * mat[1][2] * mat[2][1]
                - mat[0][1] * mat[1][2] * mat[2][3]
                - mat[0][2] * mat[1][3] * mat[2][1]
                - mat[0][3] * mat[1][1] * mat[2][2]
            ) * inv_det
        ],
        [
            (
                mat[1][0] * mat[2][3] * mat[3][2]
                + mat[1][2] * mat[2][0] * mat[3][3]
                + mat[1][3] * mat[2][2] * mat[3][0]
                - mat[1][0] * mat[2][2] * mat[3][3]
                - mat[1][2] * mat[2][3] * mat[3][0]
                - mat[1][3] * mat[2][0] * mat[3][2]
            ) * inv_det,
            (
                mat[0][0] * mat[2][2] * mat[3][3]
                + mat[0][2] * mat[2][3] * mat[3][0]
                + mat[0][3] * mat[2][0] * mat[3][2]
                - mat[0][0] * mat[2][3] * mat[3][2]
                - mat[0][2] * mat[2][0] * mat[3][3]
                - mat[0][3] * mat[2][2] * mat[3][0]
            ) * inv_det,
            (
                mat[0][0] * mat[1][3] * mat[3][2]
                + mat[0][2] * mat[1][0] * mat[3][3]
                + mat[0][3] * mat[1][2] * mat[3][0]
                - mat[0][0] * mat[1][2] * mat[3][3]
                - mat[0][2] * mat[1][3] * mat[3][0]
                - mat[0][3] * mat[1][0] * mat[3][2]
            ) * inv_det,
            (
                mat[0][0] * mat[1][2] * mat[2][3]
                + mat[0][2] * mat[1][3] * mat[2][0]
                + mat[0][3] * mat[1][0] * mat[2][2]
                - mat[0][0] * mat[1][3] * mat[2][2]
                - mat[0][2] * mat[1][0] * mat[2][3]
                - mat[0][3] * mat[1][2] * mat[2][0]
            ) * inv_det
        ],
        [
            (
                mat[1][0] * mat[2][1] * mat[3][3]
                + mat[1][1] * mat[2][3] * mat[3][0]
                + mat[1][3] * mat[2][0] * mat[3][1]
                - mat[1][0] * mat[2][3] * mat[3][1]
                - mat[1][1] * mat[2][0] * mat[3][3]
                - mat[1][3] * mat[2][1] * mat[3][0]
            ) * inv_det,
            (
                mat[0][0] * mat[2][3] * mat[3][1]
                + mat[0][1] * mat[2][0] * mat[3][3]
                + mat[0][3] * mat[2][1] * mat[3][0]
                - mat[0][0] * mat[2][1] * mat[3][3]
                - mat[0][1] * mat[2][3] * mat[3][0]
                - mat[0][3] * mat[2][0] * mat[3][1]
            ) * inv_det,
            (
                mat[0][0] * mat[1][1] * mat[3][3]
                + mat[0][1] * mat[1][3] * mat[3][0]
                + mat[0][3] * mat[1][0] * mat[3][1]
                - mat[0][0] * mat[1][3] * mat[3][1]
                - mat[0][1] * mat[1][0] * mat[3][3]
                - mat[0][3] * mat[1][1] * mat[3][0]
            ) * inv_det,
            (
                mat[0][0] * mat[1][3] * mat[2][1]
                + mat[0][1] * mat[1][0] * mat[2][3]
                + mat[0][3] * mat[1][1] * mat[2][0]
                - mat[0][0] * mat[1][1] * mat[2][3]
                - mat[0][1] * mat[1][3] * mat[2][0]
                - mat[0][3] * mat[1][0] * mat[2][1]
            ) * inv_det
        ],
        [
            (
                mat[1][0] * mat[2][2] * mat[3][1]
                + mat[1][1] * mat[2][0] * mat[3][2]
                + mat[1][2] * mat[2][1] * mat[3][0]
                - mat[1][0] * mat[2][1] * mat[3][2]
                - mat[1][1] * mat[2][2] * mat[3][0]
                - mat[1][2] * mat[2][0] * mat[3][1]
            ) * inv_det,
            (
                mat[0][0] * mat[2][1] * mat[3][2]
                + mat[0][1] * mat[2][2] * mat[3][0]
                + mat[0][2] * mat[2][0] * mat[3][1]
                - mat[0][0] * mat[2][2] * mat[3][1]
                - mat[0][1] * mat[2][0] * mat[3][2]
                - mat[0][2] * mat[2][1] * mat[3][0]
            ) * inv_det,
            (
                mat[0][0] * mat[1][2] * mat[3][1]
                + mat[0][1] * mat[1][0] * mat[3][2]
                + mat[0][2] * mat[1][1] * mat[3][0]
                - mat[0][0] * mat[1][1] * mat[3][2]
                - mat[0][1] * mat[1][2] * mat[3][0]
                - mat[0][2] * mat[1][0] * mat[3][1]
            ) * inv_det,
            (
                mat[0][0] * mat[1][1] * mat[2][2]
                + mat[0][1] * mat[1][2] * mat[2][0]
                + mat[0][2] * mat[1][0] * mat[2][1]
                - mat[0][0] * mat[1][2] * mat[2][1]
                - mat[0][1] * mat[1][0] * mat[2][2]
                - mat[0][2] * mat[1][1] * mat[2][0]
            ) * inv_det
        ]
    ]
}

