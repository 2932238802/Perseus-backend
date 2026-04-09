@echo off
set PROJECT_NAME=%~1

if "%PROJECT_NAME%"=="" (
    echo Project name is null!
    exit /b 1
)

echo creating Rust project: %PROJECT_NAME% ...
cargo new "%PROJECT_NAME%"
