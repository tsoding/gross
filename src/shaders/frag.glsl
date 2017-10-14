#version 320 es

out highp vec4 frag_color;
uniform highp vec3 color;

void main() {
    frag_color = vec4(color, 1.0);
}
