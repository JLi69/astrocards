#version 330 core

in float percy;
in vec2 tc;
uniform sampler2D tex;
uniform float alpha;
out vec4 color;

void main() {
	color = texture(tex, tc);
	float y = pow(1.0 - percy, 1.5);
	color *= vec4(y, y, y, y);
	color.a *= alpha;
}
