uniform sampler2D tex;

uniform float tick;
uniform bool crazy;

smooth in vec2 texco;
out vec4 color;


void main(void) {
    float PI = 3.14159265358979323846264;
    vec4 c = texture(tex, vec2(texco.x + sin(tick * (8 * PI) + texco.y*10)/100, texco.y));

    if(crazy) {
        c = vec4(abs(c.x * sin(tick * 19 + texco.y * 13)),
                 abs(c.y * sin(tick * 31 + texco.y * 17)),
                 abs(c.z * sin(tick * 7  + texco.y * 23)), 1);
    } else {
        c = vec4(abs(c.x * sin(tick * 19/40 + texco.y * 13)),
                 abs(c.y * sin(tick * 31/40 + texco.y * 17)),
                 abs(c.z * sin(tick * 7/40  + texco.y * 23)), 1);
    }

    float res = sin(tick) * 7.0f + 9.0f;
    c = floor(c * res) / res;
    c.a = 1.0f;

    color = c; 
}
