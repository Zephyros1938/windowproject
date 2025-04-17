#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float depthFactor; // control how fast posterization increases with depth

void main()
{
    vec4 col = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.5);

    // Simulate depth-based effect using gl_FragCoord.z (which is depth from 0.0 to 1.0)
    float depth = gl_FragCoord.z; // 0 = near, 1 = far
    float levels = mix(64.0, 0.0, depth * depthFactor); // x levels near, y levels far
    col.rgb = floor(col.rgb * levels) / levels;

    FragColor = col;
}
