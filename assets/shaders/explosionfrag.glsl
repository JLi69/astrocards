#version 330 core

in vec4 tint;
in vec2 tc;
uniform sampler2D tex;
out vec4 color;

void main() {
	color = texture(tex, tc) * tint;
}
