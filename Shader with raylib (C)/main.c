#include <raylib.h>
#include <rlgl.h>
#include <stdio.h>

#define screenWidth 400
#define screenHeight 400

#define DefaultTexture() {rlGetTextureIdDefault(), 200, 200, 1, PIXELFORMAT_UNCOMPRESSED_R8G8B8A8};

int main(void) 
{
    SetTraceLogLevel(LOG_NONE);
    InitWindow(screenWidth, screenHeight, "shader");

    const char *shaderFile = "shader.fs";
    Shader shader = LoadShader(0, shaderFile);
    long lastSavedTime = GetFileModTime(shaderFile);

    Texture2D texture = DefaultTexture();

    SetTargetFPS(60);
    while (!WindowShouldClose()) 
    {
        //hot reloading of the shader
        long currentSavedTime = GetFileModTime(shaderFile);
        if(currentSavedTime!=lastSavedTime && IsKeyReleased(KEY_R))
        {
            lastSavedTime = currentSavedTime;
            UnloadShader(shader);
            shader = LoadShader(0, shaderFile);
        }

        BeginDrawing();
            ClearBackground(SKYBLUE);
            BeginShaderMode(shader);
                DrawTexture(texture, screenWidth/2-texture.width/2, screenHeight/2-texture.height/2, YELLOW);
            EndShaderMode();
            DrawFPS(10, 10);
        EndDrawing();
    }
    UnloadShader(shader);
    CloseWindow();

    return 0;
}
