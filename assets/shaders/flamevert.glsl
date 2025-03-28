#version 330 core

layout(location = 0) in vec4 pos;

#define PI 3.1415926535

out vec2 tc;
out float percy;

uniform float time;
uniform mat4 screen;
uniform mat4 transform;

//Copied from: https://stackoverflow.com/questions/4200224/random-noise-functions-for-glsl
float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

mat4 rotationZ(float rad) {
	return mat4(
		cos(rad), -sin(rad), 0, 0,
		sin(rad), cos(rad), 0, 0,
		0, 0, 1, 0,
		0, 0, 0, 1
	);
}

void main() {
	float i = float(gl_InstanceID);

	tc = pos.xy * 0.5 + vec2(0.5, 0.5);
	tc.y = 1.0 - tc.y;
	float scale = rand(vec2(0, i * i)) * 0.2 + 0.5;
	vec4 scaled = vec4(pos.xy * scale, 0.0, 1.0);
	float theta = fract(rand(vec2(0, i * 2.0)) + time * 0.5) * 2.0 * PI;
	vec4 transformed = transform * rotationZ(theta) * scaled;

	float speed = abs(rand(vec2(i, i * cos(i)))) * 0.5 + 0.5;
	percy = fract(abs(rand(vec2(0, -i))) + fract(time / (8.0 * speed)) * 2.0);
	float y = percy * 256.0;
	float x = (rand(vec2(i, 0)) - 0.5) * 2.0 * (30.0 - percy * percy * 28.0);
	transformed += vec4(x, y, 0.0, 0.0); 
	gl_Position = screen * transformed;
}
