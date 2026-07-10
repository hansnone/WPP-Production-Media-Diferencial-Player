pub fn compute_psnr(frame_a: &[u8], frame_b: &[u8]) -> Option<f64> {
    if frame_a.len() != frame_b.len() || frame_a.is_empty() {
        return None;
    }

    let mut mse_sum = 0.0;
    // Assuming RGBA (4 channels), computing MSE across all channels or RGB only?
    // Let's compute across all channels for simplicity, or just R, G, B.
    // If it's RGBA, we can skip alpha if we want, but doing all 4 is standard for raw buffers
    // unless specified otherwise. Let's do all 4 channels to be fast and simple.

    // Process in chunks of 4 (RGBA)
    let len = frame_a.len();
    for i in 0..len {
        let diff = frame_a[i] as f64 - frame_b[i] as f64;
        mse_sum += diff * diff;
    }

    let mse = mse_sum / len as f64;

    if mse == 0.0 {
        return Some(f64::INFINITY); // Perfect match
    }

    let max_i2 = 255.0 * 255.0;
    let psnr = 10.0 * (max_i2 / mse).log10();

    Some(psnr)
}
