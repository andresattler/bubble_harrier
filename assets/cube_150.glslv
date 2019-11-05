#version 150 core
in ivec3 a_pos;
in ivec4 a_color;
out vec4 v_Color;
uniform mat4 u_model_view_proj;
void main() {
    v_Color = a_color;
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}
