struct FragmentOutput {
    @location(0) outColor: vec4<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;

fn circle(st: vec2<f32>, _radius: f32) -> f32 {
    var dist = st-vec2(0.303);

	let df = 1.-smoothstep(_radius-(_radius*0.01),
                         _radius+(_radius*0.01),
                         dot(dist,dist)*4.0);

    if (df < _radius) {
        discard;
    }

    return df;
}


@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    let st = vec2<f32>(in.position.x - 100.0,in.position.y - 173.0) / vec2<f32>(1450.0);

    let cv = circle(st,0.003);

    let out = vec4(1.0);
//
    return FragmentOutput(out);
//    return FragmentOutput(vec4<f32>(f32(1.0)));
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
//    out.color = vec3<f32>(1.0,1.0,1.0);
    out.position = camera.view_proj * vec4<f32>(model.position.x, model.position.y,f32(0), f32(1)); // 2.
    return out;
}
