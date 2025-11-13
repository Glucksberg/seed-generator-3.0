@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\nvcc.exe" -ptx -arch=sm_86 -o "C:\Users\Markus\Desktop\PROJETOS\APPs mais antigos\seed generator 3.0\seed-generator-3.0\rust\target\release\build\gpuseed-rust-e83c3f58ce44b99c\out\kernel.ptx" "C:\Users\Markus\Desktop\PROJETOS\APPs mais antigos\seed generator 3.0\seed-generator-3.0\rust\target\release\build\gpuseed-rust-e83c3f58ce44b99c\out\kernel.cu"
