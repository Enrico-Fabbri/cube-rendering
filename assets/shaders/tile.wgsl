// Vertex Shader
//struct Uniforms {
//    model_view_projection_matrix: mat4x4<f32>
//}
//@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>, 
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>
}

@vertex
fn vs_main(in: VertexInput, ) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position, 1.0); // uniforms.model_view_projection_matrix * 
    out.color = vec4<f32>(in.color, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
    return in.color;
}