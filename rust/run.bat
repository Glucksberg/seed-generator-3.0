@echo off
echo ============================================================
echo  GPU Seed Generator - Rust Edition
echo ============================================================
echo.
echo Starting GPU Seed Generator...
echo.

REM Execute the Rust program directly
.\target\release\gpuseed-rust.exe

echo.
echo ============================================================
echo Program finished. Press any key to close...
pause >nul
