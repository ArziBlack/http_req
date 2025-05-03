@echo off
echo Running Rust health_check to generate the response file...
cargo run
echo.
echo Running TypeScript to read and log the response...
npx ts-node health_check.ts
echo.
pause
