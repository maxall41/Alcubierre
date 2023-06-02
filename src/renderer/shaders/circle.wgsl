struct FragmentOutput {
    @location(0) outColor: vec4<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;

fn circle(dist: vec2<f32>, _radius: f32) -> f32 {

    let df = sqrt(dot(dist,dist));

//	let df = 1.0-smoothstep(_radius-(_radius*0.01),
//                         _radius+(_radius*0.01),
//                         dot(dist,dist)*4.0);

//    if (df < _radius) {
//            discard;
//    }

    if (df > _radius) {
        discard;
    }

    return df;
}


@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    let dist = vec2<f32>(in.model_pos.x,in.model_pos.y);

//    let cv = circle(st,0.002);
    let cv = circle(dist,in.radius);

    let out = vec4(1.0);
//
    return FragmentOutput(out);
//    return FragmentOutput(vec4<f32>(f32(1.0)));
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) radius: f32,
    @location(1) model_pos: vec2<f32>,
    @location(2) model_matrix: vec2<f32>
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) radius: f32,
    @location(3) model_matrix: vec2<f32>
};



@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
//    out.color = model.color;
    out.model_pos = vec2(model.position.x,model.position.y);
    out.radius = model.radius;
    out.model_matrix = model.model_matrix;
    out.position = camera.view_proj * vec4<f32>(model.position.x, model.position.y,f32(0), f32(1)); // 2.
    return out;
}
