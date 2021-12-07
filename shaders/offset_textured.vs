#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec4 Color;

uniform mat4 MVPMatrix;

out VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} OUT;

void main() {
    vec4 vertex = vec4(Position, 1.0);
    // calculate position by MVPMatrix
    gl_Position = MVPMatrix * vertex;

    OUT.TexCoord = TexCoord;
    OUT.Color = Color;
}
