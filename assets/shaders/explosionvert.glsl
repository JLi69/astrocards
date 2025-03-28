#version 330 core

layout(location = 0) in vec4 pos;

#define PI 3.1415926535

out vec2 tc;
out vec4 tint;
out float percy;

uniform float lifetime;
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

const float maxscale = 2.0;

void main() {
	//Instance id
	float i = float(gl_InstanceID);

	//Calculate texture coordinates
	tc = pos.xy * 0.5 + vec2(0.5, 0.5);
	tc.y = 1.0 - tc.y;

	//Scale the particle
	float scale = rand(vec2(0, i * i)) * 0.5 + 0.25;
	float scale2 = -pow((time - lifetime / 2.0) * sqrt(maxscale) * 2.0 / lifetime, 2.0) + maxscale;
	vec4 scaled = vec4(pos.xy * scale * scale2, 0.0, 1.0);

	//Calculate rotation
	float theta = fract(rand(vec2(0, i * 2.0)) + time * 0.5) * 2.0 * PI;
	vec4 transformed = transform * rotationZ(theta) * scaled;

	float speed = (abs(rand(vec2(i, i * cos(i)))) * 0.5 + 0.5) * 200.0;
	float angle = rand(vec2(0, i)) * 2.0 * PI;
	float dist = rand(vec2(sin(i) * i, i)) * 16.0 + speed * time;
	float x = dist * cos(angle);
	float y = dist * sin(angle);
	transformed += vec4(x, y, 0.0, 0.0); 
	gl_Position = screen * transformed;

	float t = sqrt(1.0 - time / lifetime);
	tint = vec4(t, t, t, t);
}
