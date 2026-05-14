#version 430 core
out vec4 frag_color;

layout(std430, binding = 0) readonly buffer Spheres {
    float data[];
} spheres;

uniform vec3 cam_pos;
uniform int n_spheres;
uniform vec2 resolution;
uniform float bound_radius;

const float SPHERE_R = 0.25;
const float FOV = 45.0;

float ray_sphere(vec3 ro, vec3 rd, vec3 center, float rv) {
    vec3 oc = ro - center;
    float b = dot(oc, rd);
    float c = dot(oc, oc) - rv * rv;
    float disc = b*b - c;
    if (disc < 0.0) return -1.0;
    float t = -b - sqrt(disc);
    return t > 0.001 ? t : -1.0;
}

void main() {
    vec2 uv = gl_FragCoord.xy / resolution - 0.5;
    float aspect = resolution.x / resolution.y;
    float half_h = tan(radians(FOV * 0.5));

    vec3 forward = normalize(-cam_pos);
    vec3 right = normalize(cross(forward, vec3(0.0, 1.0, 0.0)));
    vec3 up = cross(right, forward);

    vec3 ray_dir = normalize(forward
        + uv.x * 2.0 * aspect * half_h * right
        + uv.y * 2.0 * half_h * up);

    float t_min = 1e20;
    vec3 hit_color = vec3(0.0);
    vec3 hit_center = vec3(0.0);
    bool hit = false;

    if (length(cam_pos) > bound_radius) {
    float t_bound = ray_sphere(cam_pos, ray_dir, vec3(0.0), bound_radius);
        if (t_bound < 0.0) {
            frag_color = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
    }
    

    for (int i = 0; i < n_spheres; i++) {
        int base = i * 6;
        vec3 center = vec3(spheres.data[base], spheres.data[base+1], spheres.data[base+2]);
        float t = ray_sphere(cam_pos, ray_dir, center, SPHERE_R);
        if (t > 0.0 && t < t_min) {
            t_min = t;
            hit_color = vec3(spheres.data[base+3], spheres.data[base+4], spheres.data[base+5]);
            hit_center = center;
            hit = true;
        }
    }

    if (hit) {
        vec3 hit_pos = cam_pos + t_min * ray_dir;
        vec3 normal = normalize(hit_pos - hit_center);
        vec3 light_dir = normalize(cam_pos);
        float diffuse = max(dot(normal, light_dir), 0.0);
        vec3 reflect_dir = reflect(-light_dir, normal);
        float spec = pow(max(dot(-ray_dir, reflect_dir), 0.0), 32.0);
        frag_color = vec4(hit_color * (0.2 + diffuse * 0.7) + vec3(spec * 0.3), 1.0);
    } else {
        frag_color = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
