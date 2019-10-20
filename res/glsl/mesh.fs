smooth in vec4 fnormal;
smooth in vec2 fuv;

uniform sampler2D t_color;

out vec4 outColor;
//out vec4 outNormal;

void main()
{
    outColor = texture(t_color, fuv);
    //outNormal = fnormal;
}
