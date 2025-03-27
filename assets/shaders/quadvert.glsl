#version 330 core

layout(location = 0) in vec4 pos;

out vec2 tc;

uniform mat4 screen;
uniform mat4 transform;

void main() {
	tc = pos.xy * 0.5 + vec2(0.5, 0.5);
	tc.y = 1.0 - tc.y;
	gl_Position = screen * transform * pos;
}
