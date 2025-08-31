#include <metal_stdlib>
using namespace metal;

struct Pixel {
    float r;
    float g;
    float b;
};

struct Ray {
	float3 origin;
	float3 direction;
};

float3 ray_at(Ray r, float t) {
	return r.origin + t * r.direction;
}

bool hit_sphere(float3 center, float radius, Ray r) {
	float3 oc = r.origin - center;
	float a = dot(r.direction, r.direction);
	float b = 2.0 * dot(oc, r.direction);
	float c = dot(oc, oc) - radius*radius;
	float discriminant = b*b - 4.0 * a * c;
	return (discriminant > 0.0);
}


kernel void render_scene(
    device Pixel* outImage [[buffer(0)]],
    uint2 gid [[thread_position_in_grid]],
    uint2 grid_size [[threads_per_grid]]
) {
    uint width = grid_size.x;
    uint height = grid_size.y;
    uint idx = gid.y * width + gid.x;

    // 正規化座標
    float u = float(gid.x) / float(width);
    float v = float(gid.y) / float(height);

    // ---- アスペクト比に合わせたカメラ設定 ----
    float aspect_ratio = float(width) / float(height);
    float viewport_height = 2.0;
    float viewport_width = aspect_ratio * viewport_height;
    float focal_length = 1.0;

    float3 origin = float3(0.0, 0.0, 0.0);
    float3 horizontal = float3(viewport_width, 0.0, 0.0);
    float3 vertical   = float3(0.0, viewport_height, 0.0);
    float3 lower_left_corner =
        origin - horizontal/2 - vertical/2 - float3(0.0, 0.0, focal_length);

    // レイを生成
    Ray r;
    r.origin = origin;
    r.direction = lower_left_corner + u * horizontal + v * vertical - origin;

    // 球に当たったら赤、それ以外は背景グラデーション
    if (hit_sphere(float3(0.0, 0.0, -1.0), 0.5, r)) {
        outImage[idx].r = 1.0;
        outImage[idx].g = 0.0;
        outImage[idx].b = 0.0;
    } else {
        float3 unit_dir = normalize(r.direction);
        float t = 0.5 * (unit_dir.y + 1.0);
        float3 color = (1.0 - t) * float3(1.0, 1.0, 1.0) + t * float3(0.5, 0.7, 1.0);
        outImage[idx].r = color.r;
        outImage[idx].g = color.g;
        outImage[idx].b = color.b;
    }
}
