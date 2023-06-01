struct FragmentOutput {
    @location(0) outColor: vec4<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;

fn circle(p1: vec2<f32>,p2: vec2<f32>) -> f32 {
	return sqrt(dot(p1,p2));
}


@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    let st = vec2<f32>(in.position.x,in.position.y);

//    let cv = circle(st,0.002);

    let dist = circle(vec2(0.15,0.2),st);
    let radius = 14.0;

    if (dist > radius) {
        discard;
    }
    let d = dist / radius;

    let step = smoothstep(0.2 - 0.3,10.1, d);

    let out = mix(vec3(1.0),vec3(0.0),vec3(step));

//    let out = vec4(cv,cv,cv,1.0);
//
    return FragmentOutput(vec4(out,1.0));
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
