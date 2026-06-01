// Blit directo A→pantalla (sin letterbox ni máscara de borde) para la overlay M3.

@group(0) @binding(0) var tex_a: texture_2d<f32>;
@group(0) @binding(1) var tex_b: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32) -> VertexOut {
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
    out.uv = uvs[vert_idx];
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let uv = clamp(in.uv, vec2(0.0), vec2(1.0));
    var col = textureSample(tex_a, samp, uv);
    if col.a < 0.01 {
        col = textureSample(tex_b, samp, uv);
    }
    return vec4(col.rgb, 1.0);
}
