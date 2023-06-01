
float circle(in vec2 _st, in float _radius){
    vec2 dist = _st-vec2(0.5);
    return 1.-smoothstep(_radius-(_radius*0.01),
    _radius+(_radius*0.01),
    dot(dist,dist)*4.0);
}

layout(location = 0) out vec4 outColor;
void main(){
    vec2 st = gl_FragCoord.xy/500.0;

    vec3 color = vec3(circle(st,0.9));

    outColor = vec4( color, 1.0 );
}
