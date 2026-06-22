var<immediate> size: u32;

@group(0) @binding(0) var src_cat: texture_2d<f32>;
@group(0) @binding(1) var sam: sampler;
@group(0) @binding(2) var dst_cat: texture_storage_2d<rgba32float, write>;

@compute @workgroup_size(16, 16, 1)
fn draw(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(invocation_id.x, invocation_id.y);
    if location.x >= size || location.y >= size {
        return;
    }

    var uv = vec2(1.0, 1.0) / 3.0;

    for (var i: u32 = 0; i < countTrailingZeros(size); i += 1) {
        let isx1 = ((location.x >> i) & 1) == 1;
        let isy1 = ((location.y >> i) & 1) == 1;
        let origin = vec2(select(0.0, 0.5, isx1), select(0.0, 0.5, isy1));
        uv = origin + select(uv, -uv, isx1 && isy1) / 2.0;
    }

    var tmp = textureSampleLevel(src_cat, sam, uv, 0.0);

    textureStore(dst_cat, location, tmp);
}
