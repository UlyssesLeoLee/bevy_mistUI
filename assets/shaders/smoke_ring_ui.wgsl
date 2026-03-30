#import bevy_ui::ui_vertex_output::UiVertexOutput

struct SmokeParams {
    color: vec4<f32>,
    rect_size: vec2<f32>,
    corner_radius: vec4<f32>,
    time: f32,
    thickness: f32,
    noise_scale: f32,
    flow_speed: f32,
    breakup: f32,
    softness: f32,
};

@group(1) @binding(0) var<uniform> material: SmokeParams;

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

fn corner_radius_for_point(radii: vec4<f32>, p: vec2<f32>) -> f32 {
    if p.x >= 0.0 {
        if p.y >= 0.0 {
            return radii.z;
        }
        return radii.y;
    }
    if p.y >= 0.0 {
        return radii.w;
    }
    return radii.x;
}

fn rounded_rect_sdf(p: vec2<f32>, half_extent: vec2<f32>, radii: vec4<f32>) -> f32 {
    let max_radius = max(0.0, min(half_extent.x, half_extent.y) - 1.0);
    let radius = clamp(corner_radius_for_point(radii, p), 0.0, max_radius);
    let q = abs(p) - (half_extent - vec2<f32>(radius, radius));
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
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
    let sdf = rounded_rect_sdf(centered * px, half_extent, material.corner_radius);
    let inside_depth = max(-sdf, 0.0);
    let outside_depth = max(sdf, 0.0);

    let density = fbm(p + w1 * 3.5);
    let edge_noise = fbm(p * 1.25 - w1 * 2.9 + vec2<f32>(t * 0.25, -t * 0.18));
    let wisp_t = material.time * 0.6;
    let wisp = fbm(centered * ns * 1.5 + vec2<f32>(wisp_t, -wisp_t * 0.7));
    let filament = fbm(centered * ns * 2.2 + vec2<f32>(-wisp_t * 0.55, wisp_t * 0.8));

    let core_thickness = material.thickness * mix(0.72, 1.32, density);
    let plume_width = material.softness * 2.8 + material.breakup * 8.0;
    let boundary_offset = (edge_noise - 0.5) * material.breakup * 10.0;
    let shell_dist = abs(sdf - boundary_offset);

    let core_mask = 1.0 - smoothstep(
        core_thickness * 0.22,
        core_thickness + plume_width * 0.70,
        shell_dist
    );
    let plume_mask = 1.0 - smoothstep(
        core_thickness * 0.55,
        core_thickness + plume_width * 1.85,
        shell_dist
    );
    let inside_fade = 1.0 - smoothstep(
        core_thickness * 1.10 + plume_width * 0.20,
        core_thickness * 3.80 + plume_width * 1.60,
        inside_depth
    );
    let outside_fade = 1.0 - smoothstep(
        plume_width * 0.70,
        plume_width * 2.10 + 10.0,
        outside_depth
    );

    let smoke_core = core_mask * mix(0.62, 1.0, density);
    let smoke_plume = plume_mask * mix(0.28, 1.0, wisp) * (0.28 + material.breakup);
    let smoke =
        (smoke_core + smoke_plume * 0.82 + plume_mask * filament * 0.20 + plume_mask * edge_noise * 0.12)
        * inside_fade
        * outside_fade;
    let smoke_clamped = clamp(smoke, 0.0, 1.0);

    let scatter = 0.84 + smoke_core * 0.34 + smoke_plume * 0.18;
    let final_rgb = min(material.color.rgb * scatter, vec3<f32>(1.0));
    let alpha = smoke_clamped * material.color.a;

    return vec4<f32>(final_rgb, clamp(alpha, 0.0, 1.0));
}
