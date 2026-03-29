#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SmokeParams {
    color: vec4<f32>,
    rect_size: vec2<f32>,
    time: f32,
    thickness: f32,
    noise_scale: f32,
    flow_speed: f32,
    breakup: f32,
    softness: f32,
};

@group(2) @binding(0) var<uniform> material: SmokeParams;

fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise2d(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);

    let a = hash21(i);
    let b = hash21(i + vec2<f32>(1.0, 0.0));
    let c = hash21(i + vec2<f32>(0.0, 1.0));
    let d = hash21(i + vec2<f32>(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var v = 0.0;
    var amp = 0.5;
    var p_curr = p;
    var norm = 0.0;
    let shift = vec2<f32>(100.0, 100.0);

    for (var i = 0; i < 3; i++) {
        v += noise2d(p_curr) * amp;
        norm += amp;
        p_curr = p_curr * 2.0 + shift;
        amp *= 0.5;
    }

    return v / norm;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let centered = uv - 0.5;
    let t = material.time * material.flow_speed;
    let ns = material.noise_scale;

    let p = centered * ns;
    let w1 = vec2<f32>(
        fbm(p + vec2<f32>(t * 0.3, t * 0.7)),
        fbm(p + vec2<f32>(5.2, 1.3) + vec2<f32>(t * 0.4, t * 0.2))
    );

    let px = max(material.rect_size, vec2<f32>(1.0));
    let half_extent = px * 0.5;
    let edge_dist_px = min(
        min(uv.x * px.x, (1.0 - uv.x) * px.x),
        min(uv.y * px.y, (1.0 - uv.y) * px.y)
    );

    let density = fbm(p + w1 * 3.5);
    let edge_noise = fbm(p * 1.25 - w1 * 2.9 + vec2<f32>(t * 0.25, -t * 0.18));
    let wisp_t = material.time * 0.6;
    let wisp = fbm(centered * ns * 1.5 + vec2<f32>(wisp_t, -wisp_t * 0.7));
    let filament = fbm(centered * ns * 2.2 + vec2<f32>(-wisp_t * 0.55, wisp_t * 0.8));

    let core_thickness = material.thickness * mix(0.72, 1.32, density);
    let plume_width = material.softness * 2.8 + material.breakup * 8.0;
    let outer_mask = 1.0 - smoothstep(
        core_thickness,
        core_thickness + plume_width,
        edge_dist_px
    );
    let hollow_mask = 1.0 - smoothstep(
        0.0,
        max(core_thickness * 0.38, 1.0),
        edge_dist_px
    );
    let ring_mask = clamp(outer_mask - hollow_mask * 0.72, 0.0, 1.0);

    let plume_extent =
        core_thickness + plume_width * 1.65 + edge_noise * material.breakup * 10.0;
    let plume_mask = 1.0 - smoothstep(
        plume_extent,
        plume_extent + material.softness * 5.4 + 6.0,
        edge_dist_px
    );

    let smoke_core = ring_mask * mix(0.58, 1.0, density);
    let smoke_plume = plume_mask * mix(0.34, 1.0, wisp) * (0.34 + material.breakup);
    let smoke =
        smoke_core + smoke_plume * 0.78 + ring_mask * filament * 0.22 + plume_mask * edge_noise * 0.12;
    let smoke_clamped = clamp(smoke, 0.0, 1.0);

    let scatter = 0.84 + smoke_core * 0.34 + smoke_plume * 0.18;
    let final_rgb = min(material.color.rgb * scatter, vec3<f32>(1.0));
    let alpha = smoke_clamped * material.color.a;

    return vec4<f32>(final_rgb, clamp(alpha, 0.0, 1.0));
}
