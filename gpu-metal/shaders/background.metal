#include <metal_stdlib>
using namespace metal;

struct Pixel {
    float r;
    float g;
    float b;
};

kernel void render_background(device Pixel* outImage [[buffer(0)]],
                              uint2 gid [[thread_position_in_grid]],
                              uint2 grid_size [[threads_per_grid]]) {
    uint width = grid_size.x;
    uint height = grid_size.y;
    uint idx = gid.y * width + gid.x;

    float u = float(gid.x) / float(width - 1);
    float v = 1.0 - float(gid.y) / float(height - 1); // left top to bottom right;
    //float v = float(gid.y) / float(height - 1);

    outImage[idx].r = u;
    outImage[idx].g = v;
    outImage[idx].b = 0.0;
}

