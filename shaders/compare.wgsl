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
    scale_u_b:        f32,
    scale_v_b:        f32,
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

/// UV 0..1 del panel → sample con letterbox propio del canal.
fn zoom_pan_uv_escalado(raw_uv: vec2<f32>, su: f32, sv: f32) -> vec2<f32> {
    var centred = raw_uv - vec2(0.5, 0.5);
    centred.x = centred.x * su;
    centred.y = centred.y * sv;
    let zoomed = centred / u.zoom;
    return zoomed + vec2(0.5 + u.pan_u, 0.5 + u.pan_v);
}

/// UV compartido: A y B con el mismo encuadre (modo cortina / diff / heatmap).
fn uv_video_compartido(screen_uv: vec2<f32>) -> vec2<f32> {
    return zoom_pan_uv_escalado(screen_uv, u.scale_u, u.scale_v);
}

fn borde_uv(uv: vec2<f32>) -> f32 {
    return step(0.0, uv.x) * step(uv.x, 1.0) * step(0.0, uv.y) * step(uv.y, 1.0);
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
    let screen_uv = in.uv;

    let line_half_w = 0.0015;
    let on_left = select(screen_uv.x < u.split_pos, screen_uv.y < u.split_pos, u.split_horizontal == 1u);
    let in_line = select(
        abs(screen_uv.x - u.split_pos) < line_half_w,
        abs(screen_uv.y - u.split_pos) < line_half_w,
        u.split_horizontal == 1u
    );

    // ── Side-by-Side: dos paneles independientes ───────────────────────────
    if u.mode == 3u {
        let is_left = screen_uv.x < 0.5;
        let local = select(
            vec2(screen_uv.x * 2.0, screen_uv.y),
            vec2((screen_uv.x - 0.5) * 2.0, screen_uv.y),
            is_left,
        );
        let col_a = textureSample(
            tex_a,
            samp,
            zoom_pan_uv_escalado(local, u.scale_u, u.scale_v),
        );
        let col_b = textureSample(
            tex_b,
            samp,
            zoom_pan_uv_escalado(local, u.scale_u_b, u.scale_v_b),
        );
        let border = borde_uv(local);

        var panel: vec4<f32>;
        if is_left {
            panel = vec4(col_a.rgb * border, 1.0);
        } else if u.diff_mode == 4u {
            panel = vec4(col_b.rgb * border, 1.0);
        } else {
            let diff = compute_difference(col_a.rgb, col_b.rgb);
            panel = vec4(diff * border, 1.0);
        }

        let center_line_w = 0.0015;
        let is_center = abs(screen_uv.x - 0.5) < center_line_w;
        return select(panel, vec4(1.0, 1.0, 0.0, 1.0), is_center);
    }

    // ── Cortina / diff / heatmap: un solo plano, línea divide A | B ────────
    let uv_vid = uv_video_compartido(screen_uv);
    let col_a = textureSample(tex_a, samp, uv_vid);
    let col_b = textureSample(tex_b, samp, uv_vid);
    let border = borde_uv(uv_vid);

    var out_color: vec4<f32>;

    if u.mode == 0u {
        let base = select(col_b, col_a, on_left);
        out_color = vec4(base.rgb * border, 1.0);
    } else if u.mode == 1u {
        let diff = compute_difference(col_a.rgb, col_b.rgb);
        let base = select(vec4(diff, 1.0), col_a, on_left);
        out_color = vec4(base.rgb * border, 1.0);
    } else {
        let diff_vec = abs(col_a.rgb - col_b.rgb);
        let intensity = dot(diff_vec, vec3(0.2126, 0.7152, 0.0722)) * u.amplifier;
        let heat = heatmap_color(intensity);
        let base = select(vec4(heat, 1.0), col_a, on_left);
        out_color = vec4(base.rgb * border, 1.0);
    }

    if (in_line) {
        return vec4(1.0, 1.0, 0.0, 1.0);
    }
    return out_color;
}
