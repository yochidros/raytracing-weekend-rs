use objc2_foundation::NSString;
use objc2_metal::*;
use std::fs::File;
use std::io::Write;

#[repr(C)]
#[derive(Clone, Copy)]
struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

const METAL_FILE_NAME: &str = "raytracing.metal";
const METAL_FUNC_NAME: &str = "render_scene";

#[allow(dead_code)]
fn compute_pixels_with_shared_memory(
    width: usize,
    height: usize,
    shader_dir: &String,
) -> Vec<Pixel> {
    let pixel_count = (width * height) as usize;
    // Metal デバイス
    let device = MTLCreateSystemDefaultDevice().expect("No Metal device");

    // シェーダ読み込み
    let path = format!("{shader_dir}/{METAL_FILE_NAME}");
    let source = std::fs::read_to_string(path).unwrap();
    let source_ns = NSString::from_str(&source);
    let options = MTLCompileOptions::new();
    let lib = device
        .newLibraryWithSource_options_error(&source_ns, Some(&options))
        .expect("compile failed");

    let kernel = lib
        .newFunctionWithName(&NSString::from_str(METAL_FUNC_NAME))
        .unwrap();
    let pipeline = device
        .newComputePipelineStateWithFunction_error(&kernel)
        .unwrap();

    let buffer = device
        .newBufferWithLength_options(
            pixel_count * std::mem::size_of::<Pixel>(),
            MTLResourceOptions::StorageModeShared,
        )
        .expect("buffer alloc failed");

    // コマンド
    let queue = device.newCommandQueue().expect("queue failed");
    let command_buffer = queue.commandBuffer().unwrap();
    let encoder = command_buffer.computeCommandEncoder().unwrap();

    // --- エンコード
    encoder.setComputePipelineState(&pipeline);
    unsafe {
        encoder.setBuffer_offset_atIndex(Some(&buffer), 0, 0);
    }

    let grid_size = MTLSize {
        width,
        height,
        depth: 1,
    };
    let threadgroup_size = MTLSize {
        width: 8,
        height: 8,
        depth: 1,
    };
    encoder.dispatchThreads_threadsPerThreadgroup(grid_size, threadgroup_size);
    encoder.endEncoding();

    command_buffer.commit();
    unsafe {
        command_buffer.waitUntilCompleted();
    }

    // CPU にコピー
    let ptr: std::ptr::NonNull<Pixel> = buffer.contents().cast();
    unsafe { std::slice::from_raw_parts(ptr.as_ptr(), pixel_count).to_vec() }
}

#[allow(dead_code)]
fn compute_pixels_with_private_memory(
    width: usize,
    height: usize,
    shader_dir: &String,
) -> Vec<Pixel> {
    let pixel_count = (width * height) as usize;
    // Metal デバイス
    let device = MTLCreateSystemDefaultDevice().expect("No Metal device");

    // シェーダ読み込み
    let path = format!("{shader_dir}/{METAL_FILE_NAME}");
    let source = std::fs::read_to_string(path).unwrap();
    let source_ns = NSString::from_str(&source);
    let options = MTLCompileOptions::new();
    let lib = device
        .newLibraryWithSource_options_error(&source_ns, Some(&options))
        .expect("compile failed");

    let kernel = lib
        .newFunctionWithName(&NSString::from_str(METAL_FUNC_NAME))
        .unwrap();
    let pipeline = device
        .newComputePipelineStateWithFunction_error(&kernel)
        .unwrap();

    // GPU 専用バッファ（CPU から直接触れない）
    let gpu_buffer = device
        .newBufferWithLength_options(
            pixel_count * std::mem::size_of::<Pixel>(),
            MTLResourceOptions::StorageModePrivate,
        )
        .expect("gpu buffer alloc failed");

    // CPU 共有バッファ（結果コピー用）
    let cpu_buffer = device
        .newBufferWithLength_options(
            pixel_count * std::mem::size_of::<Pixel>(),
            MTLResourceOptions::StorageModeShared,
        )
        .expect("cpu buffer alloc failed");

    // コマンド
    let queue = device.newCommandQueue().expect("queue failed");
    let command_buffer = queue.commandBuffer().unwrap();
    let encoder = command_buffer.computeCommandEncoder().unwrap();

    // --- エンコード (GPU 側に書き込み)
    encoder.setComputePipelineState(&pipeline);
    unsafe {
        encoder.setBuffer_offset_atIndex(Some(&gpu_buffer), 0, 0);
    }

    let grid_size = MTLSize {
        width,
        height,
        depth: 1,
    };
    let threadgroup_size = MTLSize {
        width: 8,
        height: 8,
        depth: 1,
    };
    encoder.dispatchThreads_threadsPerThreadgroup(grid_size, threadgroup_size);
    encoder.endEncoding();

    // --- Blit (GPU → CPU 転送)
    let blit_encoder = command_buffer.blitCommandEncoder().unwrap();
    unsafe {
        blit_encoder.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
            &gpu_buffer,
            0, // src + offset
            &cpu_buffer,
            0,                                          // dst + offset
            pixel_count * std::mem::size_of::<Pixel>(), // size
        );
    }
    blit_encoder.endEncoding();

    command_buffer.commit();
    unsafe {
        command_buffer.waitUntilCompleted();
    }

    // CPU バッファの内容を読む
    let ptr: std::ptr::NonNull<Pixel> = cpu_buffer.contents().cast();
    unsafe { std::slice::from_raw_parts(ptr.as_ptr(), pixel_count).to_vec() }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        panic!("Usage: cargo run -p gpu-metal shared-dir-path");
    }

    let shader_dir = &argv[1];
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    // let pixels = compute_pixels_with_shared_memory(width, height, shader_dir);
    let pixels = compute_pixels_with_private_memory(image_width, image_height, shader_dir);

    // PPM 出力
    let mut file = File::create("test_gpu.ppm").unwrap();
    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", image_width, image_height).unwrap();
    writeln!(file, "255").unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let idx = j * image_width + i;
            let p = pixels[idx];
            let ir = (255.999 * p.r) as i32;
            let ig = (255.999 * p.g) as i32;
            let ib = (255.999 * p.b) as i32;
            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    println!("✅ Done");
}
