#version 330 core

in vec2 tc;
out vec4 color;

uniform sampler2D tex;
uniform vec4 tint;

void main() {
	color = texture(tex, tc) * tint;
}
