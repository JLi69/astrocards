#version 330 core

in vec2 tc;
out vec4 color;

void main() {
	color = vec4(tc.x, tc.y, 0.0, 1.0);
}
