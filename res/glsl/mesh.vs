uniform mat4 matrix;

in vec3 position;
in vec3 normal;
in vec2 uv;

smooth out vec2 fuv;
smooth out vec4 fnormal;

void main()
{
    gl_Position = matrix * vec4(position, 1.0f);
    fuv = uv;
    fnormal = matrix * vec4(normal, 1.0f);
}
