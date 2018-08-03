#version 120

#ifdef GL_ES
precision mediump float;
#endif
//uniform sampler2D t_reflectance;
//uniform vec4 i_ambient;
//varying float v_diffuse;
varying vec4 v_color; // 顶点颜色
varying vec2 v_texcoord;

uniform sampler2D u_texture;


void main (void)
{
    vec4 texture_color;

//vec4 color = texture2D(t_reflectance, v_texcoord);
//这里分解开来是 color*vec3(1,1,1)*v_diffuse + color*i_ambient
//色*光*夹角cos + 色*环境光
    texture_color = texture2D(u_texture, v_texcoord);
    gl_FragColor = mix(v_color, texture_color, texture_color.a);

//v_color;   //color*(vec4(v_diffuse) + i_ambient);
}