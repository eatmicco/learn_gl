#version 330 core

uniform sampler2D Texture;

in VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} IN;

out vec4 Color;

void main() {
    vec4 textureColor = texture(Texture, IN.TexCoord);
    Color = textureColor * IN.Color;
}