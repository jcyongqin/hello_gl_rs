#version 120

//uniform mat4 mvp_matrix; //透视矩阵 * 视图矩阵 * 模型变换矩阵
//uniform mat3 normal_matrix; //法线变换矩阵(用于物体变换后法线跟着变换)
//uniform vec3 ec_light_dir; //光照方向
attribute vec3 a_vertex; // 顶点坐标
attribute vec3 a_color; // 顶点颜色
attribute vec2 a_texcoord; //纹理坐标

//attribute vec3 a_normal; //顶点法线
//varying float v_diffuse; //法线与入射光的夹角
varying vec2 v_texcoord; //2d纹理坐标
varying vec4 v_color; // 顶点颜色
void main(void)
{
 //归一化法线
 //vec3 ec_normal = normalize(normal_matrix * a_normal);
 //v_diffuse 是法线与光照的夹角.根据向量点乘法则,当两向量长度为1是 乘积即cosθ值
 //v_diffuse = max(dot(ec_light_dir, ec_normal), 0.0);
 //v_texcoord = a_texcoord;
 gl_Position = /*mvp_matrix * */ vec4(a_vertex, 1.0);
 v_color = vec4(a_color, 1.0);
 v_texcoord = a_texcoord;
}