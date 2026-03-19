// DiffPlayerQC — shader de comparación A/B (cortina, diff, heatmap, lado a lado).
// Debe coincidir con `ShaderUniforms` en `src/renderer.rs` (tamaños y orden de campos).

// ---------------------------------------------------------------------------
//  Uniform buffer (must match ShaderUniforms in renderer.rs)
// ---------------------------------------------------------------------------
struct Uniforms {
    split_pos:        f32,
    mode:             u32,
    diff_mode:        u32,
    amplifier:        f32,
    zoom:             f32,
    pan_u:            f32,
    pan_v:            f32,
    scale_u:          f32,
    scale_v:          f32,
    bg_r:             f32,
    bg_g:             f32,
    bg_b:             f32,
    split_horizontal: u32,
}

@group(0) @binding(0) var tex_a:   texture_2d<f32>;
@group(0) @binding(1) var tex_b:   texture_2d<f32>;
@group(0) @binding(2) var samp:    sampler;
@group(0) @binding(3) var<uniform> u: Uniforms;

// ---------------------------------------------------------------------------
//  Vertex stage — generates a fullscreen triangle from vertex index
//  (no vertex buffer required)
// ---------------------------------------------------------------------------
struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0)       uv:  vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32) -> VertexOut {
    // Fullscreen triangle trick: three hard-coded clip-space positions
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -3.0),
        vec2(-1.0,  1.0),
        vec2( 3.0,  1.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2(0.0, 2.0),
        vec2(0.0, 0.0),
        vec2(2.0, 0.0),
    );

    var out: VertexOut;
    out.pos = vec4(positions[vert_idx], 0.0, 1.0);
    out.uv  = uvs[vert_idx];
    return out;
}

// ---------------------------------------------------------------------------
//  Fragment stage — the comparison logic
// ---------------------------------------------------------------------------

/// Apply zoom and pan to a raw UV coordinate.
fn zoom_pan_uv(raw_uv: vec2<f32>) -> vec2<f32> {
    // Zoom around centre (0.5, 0.5)
    var centred = raw_uv - vec2(0.5, 0.5);
    
    // Apply aspect ratio scale (letterboxing)
    centred.x = centred.x * u.scale_u;
    centred.y = centred.y * u.scale_v;
    
    let zoomed  = centred / u.zoom;
    // Apply pan offset (pan_u, pan_v are in UV space)
    return zoomed + vec2(0.5 + u.pan_u, 0.5 + u.pan_v);
}

/// Map a scalar intensity (0–1) to heatmap color.
/// 0.0 = black, 0.25 = blue, 0.5 = yellow, 0.75 = orange, 1.0 = red
fn heatmap_color(t: f32) -> vec3<f32> {
    let c = clamp(t, 0.0, 1.0);
    // Gradient: black → dark-blue → yellow → red
    let r = smoothstep(0.4, 0.8, c);
    let g = 1.0 - smoothstep(0.5, 1.0, c) * (1.0 - smoothstep(0.0, 0.4, c));
    let b = smoothstep(0.0, 0.25, c) * (1.0 - smoothstep(0.25, 0.6, c));
    return vec3(r, g, b);
}

// Computes the configured difference mode (0=Legacy, 1=Linear, 2=Sqrt, 3=Signed)
fn compute_difference(col_a: vec3<f32>, col_b: vec3<f32>) -> vec3<f32> {
    var diff: vec3<f32>;
    if u.diff_mode == 0u {
        // LegacyAbs: saturate(abs(A-B) * 2)
        diff = clamp(abs(col_a - col_b) * 2.0, vec3(0.0), vec3(1.0));
    } else if u.diff_mode == 1u {
        // AbsLinear: saturate(abs(A-B) * AMP)
        diff = clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0));
    } else if u.diff_mode == 2u {
        // AbsSqrt: sqrt(saturate(abs(A-B) * AMP))
        diff = sqrt(clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0)));
    } else {
        // SignedDiverging
        let mag = sqrt(clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0)));
        let is_positive = step(vec3(0.0), col_a - col_b);
        diff = mix(vec3(0.5) - mag * 0.5, vec3(0.5) + mag * 0.5, is_positive);
    }
    return diff;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // Transform UV with zoom/pan
    let uv = zoom_pan_uv(in.uv);

    // If UV is out of [0,1] range due to pan, show a dark border
    let border = step(0.0, uv.x) * step(uv.x, 1.0) * step(0.0, uv.y) * step(uv.y, 1.0);

    let col_a = textureSample(tex_a, samp, uv);
    let col_b = textureSample(tex_b, samp, uv);

    var out_color: vec4<f32>;

    let line_half_w = 0.0015;
    // Curtain orientation: 0 = vertical (split on X), 1 = horizontal (split on Y)
    let on_left = select(in.uv.x < u.split_pos, in.uv.y < u.split_pos, u.split_horizontal == 1u);
    let in_line = select(
        abs(in.uv.x - u.split_pos) < line_half_w,
        abs(in.uv.y - u.split_pos) < line_half_w,
        u.split_horizontal == 1u
    );

    if u.mode == 0u {
        // ── 0: Split-Screen (curtain) ──────────────────────────────────────
        let base = select(col_b, col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else if u.mode == 1u {
        // ── 1: Absolute Difference ─────────────────────────────────────────
        let diff = compute_difference(col_a.rgb, col_b.rgb);
        let base = select(vec4(diff, 1.0), col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else if u.mode == 2u {
        // ── 2: Heatmap QC ──────────────────────────────────────────────────
        let diff_vec = abs(col_a.rgb - col_b.rgb);
        // Perceptual luminance weight
        let intensity = dot(diff_vec, vec3(0.2126, 0.7152, 0.0722)) * u.amplifier;
        let heat      = heatmap_color(intensity);
        let base = select(vec4(heat, 1.0), col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else {
        // ── 3: Side-by-Side ────────────────────────────────────────────────
        // Left half shows tex_a scaled to hit 0..1 in x
        // Right half shows tex_b scaled to hit 0..1 in x
        let is_left_half = in.uv.x < 0.5;
        
        var sbs_uv = in.uv;
        if is_left_half {
            sbs_uv.x = sbs_uv.x * 2.0;
        } else {
            sbs_uv.x = (sbs_uv.x - 0.5) * 2.0;
        }
        
        sbs_uv = zoom_pan_uv(sbs_uv);
        
        let sbs_col_a = textureSample(tex_a, samp, sbs_uv);
        let sbs_col_b = textureSample(tex_b, samp, sbs_uv);
        
        var right_side: vec4<f32>;
        if u.diff_mode == 4u {
            right_side = sbs_col_b;
        } else {
            right_side = vec4(compute_difference(sbs_col_a.rgb, sbs_col_b.rgb), 1.0);
        }
        
        let base = select(right_side, sbs_col_a, is_left_half);
        
        // Draw a line down the middle
        let center_line_w = 0.0015;
        let is_center = abs(in.uv.x - 0.5) < center_line_w;
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), is_center);
        
        // Disable outer border clipping for side-by-side mode 
        // because we manually handle the UV scaling and we don't want the 0.5 split clipping it.
        // Instead, we just check if the transformed sbs_uv is out of bounds [0, 1].
        let sbs_border = step(0.0, sbs_uv.x) * step(sbs_uv.x, 1.0) * step(0.0, sbs_uv.y) * step(sbs_uv.y, 1.0);
        let bg = vec3(u.bg_r, u.bg_g, u.bg_b);
        return vec4(mix(bg, out_color.rgb, sbs_border), 1.0);
    }

    // Mix with background color (out of video UV range)
    let bg = vec3(u.bg_r, u.bg_g, u.bg_b);
    return vec4(mix(bg, out_color.rgb, border), 1.0);
}
