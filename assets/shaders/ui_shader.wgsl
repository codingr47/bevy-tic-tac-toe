#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let color = vec4(1.0,0.0,0.0,1.0);
    //let color = textureSample(my_texture, my_sampler, in.uv);
    return color;
}