#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var my_texture: texture_2d<f32>;

@group(1) @binding(1)
var my_sampler: sampler;

struct TimeUniform {
    time: f32,
};

@group(1) @binding(2)
var<uniform> timeUniform: TimeUniform;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    if (timeUniform.time == 0.0) {
        return textureSample(my_texture, my_sampler, in.uv);
    } else {
        let color = textureSample(my_texture, my_sampler, in.uv);

        return vec4(color.x, color.y, color.z, abs(tan(timeUniform.time))); 
    }
}