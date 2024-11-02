/*
MIT License

Copyright (c) 2024 jess-hammer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

// From https://github.com/jess-hammer/2d-shadows-demo-godot
// With slight modifications


shader_type canvas_item;
render_mode unshaded;

uniform vec4 color : source_color; 
uniform float angle : hint_range(0,360); 
uniform float max_dist : hint_range(0,1000) = 100; 
uniform sampler2D gradientTexture;

vec4 get_gradient_color(float position) { 
    return texture(gradientTexture, vec2(position, 0.5)); 
}

void fragment() { 
    float ang_rad = angle * 3.1416 / 360.0; 
    vec2 dir = vec2(sin(ang_rad),cos(ang_rad));
	dir.x = -dir.x; 
    vec2 at = screen_uv_to_sdf(SCREEN_UV); 
    float accum = 0.0;
    while(accum < max_dist) {
        float d = texture_sdf(at);
        accum+=d;
        if (d < 0.01) {
            break;
        }
        at += d * dir;
    }
    float alpha = 1.0-min(1.0,accum/max_dist);
    
    // the gradient controls the falloff of the shadow
    alpha = get_gradient_color(alpha).r;
    
    COLOR = vec4(color.rgb,alpha * color.a);
}