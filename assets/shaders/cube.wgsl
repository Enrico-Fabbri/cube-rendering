// Vertex
struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct RenderInput {
    @location(9) model_render: u32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) model_position: vec4<f32>,
    // 
    @location(0) color: vec3<f32>,
    @location(1) position: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput, render: RenderInput) -> VertexOutput {
    var out: VertexOutput;

    if render.model_render == 1u {   
        let model_matrix = mat4x4<f32>(
            instance.model_matrix_0,
            instance.model_matrix_1,
            instance.model_matrix_2,
            instance.model_matrix_3,
        );

        out.model_position = camera.view_proj * model_matrix *  vec4<f32>(model.position, 1.0);
        out.color = vec3<f32>(0.3, 0.7, 0.4);
        out.position = model.position;
    }
    
    return out;
}

fn is_border(x: f32, y: f32, z: f32, border: f32, dimension: f32) -> bool {
    let min = border;
    let max = dimension - border;
    let x_min = min - x;
    let x_max = max - x;
    let y_min = min - y;
    let y_max = max - y;
    let z_min = min - z;
    let z_max = max - z;

    if  x_min > 0.0 && y_min > 0.0 || x_max < 0.0 && y_max < 0.0 ||
        x_min > 0.0 && z_min > 0.0 || x_max < 0.0 && z_max < 0.0 ||
        y_min > 0.0 && z_min > 0.0 || y_max < 0.0 && z_max < 0.0 ||
        x_max < 0.0 && y_min > 0.0 || x_min > 0.0 && y_max < 0.0 ||
        x_max < 0.0 && z_min > 0.0 || x_min > 0.0 && z_max < 0.0 ||
        y_max < 0.0 && z_min > 0.0 || y_min > 0.0 && z_max < 0.0 {
        return true;
    } else {
        return false;
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if is_border(in.position.x, in.position.y, in.position.z, 0.02, 1.0) {
        return vec4<f32>(in.color.x + 0.1, in.color.y + 0.1, in.color.z + 0.1, 1.0);
    } else {
        return vec4<f32>(in.color, 1.0);
    }
}
