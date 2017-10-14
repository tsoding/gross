#version 320 es

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_color;

uniform float angle;

out vec3 color;

void main() {
    mat3 rotation_mat = mat3(
        cos(angle),  sin(angle), 0,
        -sin(angle), cos(angle), 0,
        0,           0,          1
        );

    color = vertex_color;
    gl_Position = vec4(rotation_mat * vertex_position, 1.0);
}
