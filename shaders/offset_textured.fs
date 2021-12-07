#version 330 core

uniform sampler2D Texture;
uniform vec2 Offset;

in VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} IN;

out vec4 Color;

void main() {
    vec2 coord = IN.TexCoord + Offset;
    vec4 textureColor = texture(Texture, coord);
    Color = textureColor * IN.Color;
}