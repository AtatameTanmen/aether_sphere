@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> size: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_texture: texture_2d<f32>;

@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var uv2 = uv;
    var loc = vec2<u32>(0, 0);

    for (var i: u32 = 0; i < countTrailingZeros(size); i += 1) {
        const HALF = vec2(0.5, 0.5);
        let isx1 = uv2.x > 0.5;
        let isy1 = uv2.y > 0.5;
        let isc = uv2.x + uv2.y > 0.5;

        uv2 = 2.0 * select(select(select(uv2, HALF - uv2, isc), vec2(uv2.x, uv2.y - 0.5), isy1), vec2(uv2.x - 0.5, uv2.y), isx1);
        loc = select(select(select(vec2<u32>(loc.x << 1, loc.y << 1), vec2<u32>((loc.x << 1) | 1, (loc.y << 1) | 1), isc), vec2<u32>(loc.x << 1, (loc.y << 1) | 1), isy1), vec2<u32>((loc.x << 1) | 1, loc.y << 1), isx1);
    }

    return textureLoad(base_texture, loc, 0);
}
