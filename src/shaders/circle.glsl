#version 320 es

out highp vec4 frag_color;
uniform highp float radius;

/**
 * @author jonobr1 / http://jonobr1.com/
 */

/**
 * Convert r, g, b to normalized vec3
 */
highp vec3 rgb(highp float r, highp float g, highp float b) {
    return vec3(r / 255.0, g / 255.0, b / 255.0);
}

/**
 * Draw a circle at vec2 `pos` with radius `rad` and
 * color `color`.
 */
highp vec4 circle(highp vec2 uv, highp vec2 pos, highp float rad, highp vec3 color) {
    highp float d = length(pos - uv) - rad;
    highp float t = clamp(d, 0.0, 1.0);
    return vec4(color, 1.0 - t);
}

void main() {

    highp vec2 uv = gl_FragCoord.xy;
    highp vec2 center = vec2(400.0, 300.0);

    // Background layer
    // highp vec4 layer1 = vec4(rgb(210.0, 222.0, 228.0), 1.0);
    highp vec4 layer1 = vec4(0.0, 0.0, 0.0, 0.0);

    // Circle
    highp vec3 red = rgb(225.0, 95.0, 60.0);
    highp vec4 layer2 = circle(uv, center, radius, red);

    // Blend the two
    frag_color = mix(layer1, layer2, layer2.a);
}
