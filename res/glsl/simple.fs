uniform sampler2D tex;

uniform bool crazy;
uniform bool boring;
uniform float tick;

smooth in vec2 texco;
out vec4 color;

void main(void) {
    float PI = 3.14159265358979323846264;
    vec4 c = vec4(1);

    // used in place of 'pow' function, since it is undefined if x < 0
    // I am doing this so then the screen wavy stops at the edge of the screen.
    // this prevents the output from looping from left side to right and back during the sin wiggles
    float x = texco.x*2.0f-1.0f;
    float xx = x * x;
    float xxxx = xx * xx;
    float texcox = -xxxx+1.0f;

    if(crazy) {
        c = texture(tex, vec2(texco.x + texcox * sin(tick*PI*16 + texco.y*16)/100, texco.y));
        c.r = c.g + color.b;
        c.g = sin(tick * 23) * c.r + sin(tick * 5) * c.g + sin(tick * 11) * c.b;

        if(c.r < 0.5) c.b = 0.2;
        else c.b = 0.7f;
    } else if(boring) {
        c = texture(tex, texco);
    } else {
        c = texture(tex, 
                    vec2(texco.x + texcox * sin(tick*20 + texco.y*20)/100, texco.y));
    }

    color = c + vec4(0.2, 0.0, 0.0, 0.0);
}
