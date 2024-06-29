#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

void main() 
{
    float r1 = 0.1;
    float r2 = 0.5;
    float distanceFromCentre = distance(fragTexCoord, vec2(0.5));

    float in_r1 = step(distanceFromCentre, r1);             //inside r1
    float in_r2 = step(distanceFromCentre, r2) - in_r1;     //inside r2 but outside r1 (exclusive to r2)
    
    float distance_from_r1 = distanceFromCentre-r1;
    float alpha = in_r2*(1.0-distance_from_r1/(r2-r1)) + in_r1;

    finalColor = vec4(fragColor.xyz, pow(alpha, 2.0)); 
}
