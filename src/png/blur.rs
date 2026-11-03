//! Approximate Gaussian blur: three passes of an extended box blur.
//!
//! This follows the method Pillow uses for `ImageFilter.GaussianBlur`
//! (the radius is the standard deviation). Edge pixels are repeated.

const PASSES: usize = 3;

/// Blur a single-channel float plane in place.
pub fn gaussian_blur(plane: &mut [f32], width: usize, height: usize, sigma: f32) {
    let radius = box_radius(sigma);
    let mut scratch = vec![0.0f32; width.max(height)];
    let mut column = vec![0.0f32; height];

    for _ in 0..PASSES {
        for row in plane.chunks_exact_mut(width) {
            box_blur_line(row, radius, &mut scratch);
        }
        for x in 0..width {
            for (y, v) in column.iter_mut().enumerate() {
                *v = plane[y * width + x];
            }
            box_blur_line(&mut column, radius, &mut scratch);
            for (y, v) in column.iter().enumerate() {
                plane[y * width + x] = *v;
            }
        }
    }
}

/// Fractional box radius whose three passes approximate a Gaussian.
fn box_radius(sigma: f32) -> f32 {
    let var = sigma * sigma / PASSES as f32;
    let ideal = (12.0 * var + 1.0).sqrt();
    let l = ((ideal - 1.0) / 2.0).floor();
    let a = (2.0 * l + 1.0) * (l * (l + 1.0) - 3.0 * var);
    let a = a / (6.0 * (var - (l + 1.0) * (l + 1.0)));
    l + a
}

/// One box-blur pass over `data`. `scratch` must be at least as long as `data`.
fn box_blur_line(data: &mut [f32], radius: f32, scratch: &mut [f32]) {
    let n = data.len();
    let src = &mut scratch[..n];
    src.copy_from_slice(data);

    let l = radius.floor() as isize;
    let frac = radius - l as f32;
    let norm = 1.0 / (2.0 * l as f32 + 1.0 + 2.0 * frac);
    let at = |i: isize| src[i.clamp(0, n as isize - 1) as usize];

    // Sum over the taps -l..=l around position 0.
    let mut sum: f32 = (-l..=l).map(at).sum();
    for i in 0..n as isize {
        let edge = at(i - l - 1) + at(i + l + 1);
        data[i as usize] = (sum + frac * edge) * norm;
        // Move the window to position i + 1.
        sum += at(i + l + 1) - at(i - l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_plane_stays_constant() {
        let mut p = vec![0.5f32; 40 * 30];
        gaussian_blur(&mut p, 40, 30, 5.0);
        assert!(p.iter().all(|v| (v - 0.5).abs() < 1e-4));
    }

    #[test]
    fn impulse_spreads_and_keeps_mass() {
        let (w, h) = (101, 101);
        let mut p = vec![0.0f32; w * h];
        p[50 * w + 50] = 1.0;
        gaussian_blur(&mut p, w, h, 4.0);
        let total: f32 = p.iter().sum();
        assert!((total - 1.0).abs() < 1e-3);
        assert!(p[50 * w + 50] < 0.1);
    }
}
