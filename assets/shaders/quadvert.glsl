#version 330 core

layout(location = 0) in vec4 pos;

out vec2 tc;

void main() {
	tc = pos.xy * 0.5 + vec2(0.5, 0.5);
	gl_Position = pos;
}
