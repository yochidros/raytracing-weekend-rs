#include <metal_stdlib>
using namespace metal;

float rand(uint2 seed) {
    seed = seed ^ (seed.yx * 1664525u + 1013904223u);
    uint bits = (seed.x ^ seed.y) & 0x007FFFFFu | 0x3F800000u;
    return as_type<float>(bits) - 1.0;
}

struct Pixel {
    float r;
    float g;
    float b;
};

struct Sphere {
    float3 center;
    float radius;
};

struct Ray {
	float3 origin;
	float3 direction;
};

struct HitRecord {
  float3 p;
  float3 normal;
  float t;
  bool hit;
};

HitRecord hit_sphere(float3 center, float radius, Ray r) {
    HitRecord rec;
    rec.hit = false;

    float3 oc = r.origin - center;
    float a = dot(r.direction, r.direction);
    float h = dot(oc, r.direction);
    float c = dot(oc, oc) - radius*radius;
    float discriminant = h*h - a*c;

    if (discriminant < 0.0) {
        return rec;
    }

    float t = (-h - sqrt(discriminant)) / a;
    if (t > 0.0) {
        rec.t = t;
        rec.p = r.origin + t * r.direction;
        rec.normal = normalize(rec.p - center);
        rec.hit = true;
    }
    return rec;
}


kernel void render_scene(
    device Pixel* outImage [[buffer(0)]],
    uint2 gid [[thread_position_in_grid]],
    uint2 grid_size [[threads_per_grid]]
) {
  uint width = grid_size.x;
  uint height = grid_size.y;
  uint idx = gid.y * width + gid.x;

  int samples_per_pixel = 50;

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

  Sphere spheres[2];
  spheres[0].center = float3(0.0, 0.0, -1.0);
  spheres[0].radius = 0.5;

  spheres[1].center = float3(0.0, -100.5, -1.0); // 地面っぽい大きな球
  spheres[1].radius = 100.0;

  float3 final_color = float3(0.0, 0.0, 0.0);

  for (int s = 0; s < samples_per_pixel; s++) {
    float ru = rand(gid + uint2(gid.x, s));
    float rv = rand(gid + uint2(gid.y, s*17));
    float u = (float(gid.x) + ru) / float(width);
    float v = (float(gid.y) + rv) / float(height);

    // レイを生成
    Ray r;
    r.origin = origin;
    r.direction = lower_left_corner + u * horizontal + v * vertical - origin;

    HitRecord closest_rec;
    closest_rec.hit = false;
    closest_rec.t = 1e9;

    for (int i = 0; i < 2; i++) {
      HitRecord rec = hit_sphere(spheres[i].center, spheres[i].radius, r);
      if (rec.hit && rec.t < closest_rec.t) {
        closest_rec = rec;
      }
    }

    float3 sample_color;
    if (closest_rec.hit) {
      // 法線を色として可視化
      sample_color = 0.5 * (closest_rec.normal + float3(1.0, 1.0, 1.0));
    } else {
      // background
      float3 unit_dir = normalize(r.direction);
      float t = 0.5 * (unit_dir.y + 1.0);
      sample_color = (1.0 - t) * float3(1.0, 1.0, 1.0) + t * float3(0.5, 0.7, 1.0);
    }

    final_color += sample_color;
  }

  final_color /= float(samples_per_pixel);

  outImage[idx].r = final_color.r;
  outImage[idx].g = final_color.g;
  outImage[idx].b = final_color.b;
}
