@fragment
fn fragment(@location(2) uv: vec2<f32>,) -> @location(0) vec4<f32> {
    var uv2 = uv;
    let c0 = vec4(0.0, 0.0, 0.0, 1.0);
    let c1 = vec4(1.0, 0.0, 0.0, 1.0);
    let c2 = vec4(0.0, 1.0, 0.0, 1.0);
    let c3 = vec4(0.0, 0.0, 1.0, 1.0);

    for(var i = 0; i < 4; i += 1) {
        uv2 = 2.0 * select(select(select(uv2, vec2(0.5 - uv2.x, 0.5 - uv2.y), uv2.x + uv2.y > 0.5), vec2(uv2.x, uv2.y - 0.5), uv2.y > 0.5), vec2(uv2.x - 0.5, uv2.y), uv2.x > 0.5);
    }

    return select(select(select(c1, c0, uv2.x + uv2.y > 0.5), c3, uv2.y > 0.5), c2, uv2.x > 0.5);
}
