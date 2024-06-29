mod raylib;
use raylib::*;

fn main() {
    unsafe {
        assign_cstring!(title, "sample");
        assign_cstring!(text, "Hello, World");

        SetTraceLogLevel(TraceLogLevel::LOG_NONE);
        InitWindow(800, 450, title.as_ptr());

        SetTargetFPS(60);
        while !WindowShouldClose() {
            BeginDrawing();
                ClearBackground(Color::VIOLET);
                DrawText(text.as_ptr(), 190, 200, 50, Color::BLACK);
                DrawFPS(10, 10);
            EndDrawing();
        }
        CloseWindow();
    }
}