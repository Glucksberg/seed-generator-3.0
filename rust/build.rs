// build.rs - Compile CUDA kernel to PTX when CUDA Toolkit is available
use std::env;
use std::path::PathBuf;
use std::process::Command;

// Helper function to find nvcc in common CUDA installation paths
fn find_nvcc() -> Option<String> {
    // Common CUDA installation paths on Windows
    let cuda_paths = vec![
        r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\nvcc.exe",
        r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.0\bin\nvcc.exe",
        r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v11.8\bin\nvcc.exe",
    ];
    
    for path in cuda_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    
    None
}

// Helper function to find Visual Studio vcvars64.bat
// This is needed because nvcc requires cl.exe (Visual Studio C++ compiler) to be in PATH
fn find_vcvars64() -> Option<String> {
    // Common Visual Studio installation paths
    let vs_paths = vec![
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Auxiliary\Build\vcvars64.bat",
    ];
    
    for path in vs_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    
    None
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let module_path = PathBuf::from(&out_dir).join("kernel_ptx.rs");
    
    // Try to find nvcc in common CUDA installation paths
    let nvcc_path = find_nvcc();
    
    // Check if CUDA Toolkit is installed
    let nvcc_output = if let Some(ref path) = nvcc_path {
        Command::new(path)
            .arg("--version")
            .output()
    } else {
        Command::new("nvcc")
            .arg("--version")
            .output()
    };
    
    let ptx_available = if nvcc_output.is_ok() {
        println!("cargo:warning=CUDA Toolkit detected. Kernel compilation will be attempted.");
        
        // CUDA kernel source using cuRAND for true GPU random number generation
        let kernel_source = r#"
#include <curand_kernel.h>

// Kernel to initialize cuRAND states
extern "C" __global__ void init_curand_states(
    curandState* states,
    unsigned long long seed,
    int num_states
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_states) {
        curand_init(seed, idx, 0, &states[idx]);
    }
}

// Kernel to generate random bytes using cuRAND
extern "C" __global__ void generate_random_bytes(
    curandState* states,
    unsigned char* output,
    int num_elements
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_elements) {
        curandState local_state = states[idx];
        
        // Generate 16 bytes per element using cuRAND
        for (int i = 0; i < 16; i++) {
            // Generate random unsigned int and take lower 8 bits
            unsigned int rand_val = curand(&local_state);
            output[idx * 16 + i] = (unsigned char)(rand_val & 0xFF);
        }
        
        // Save state back
        states[idx] = local_state;
    }
}
"#;
        
        // Write kernel to temporary file
        let kernel_path = PathBuf::from(&out_dir).join("kernel.cu");
        std::fs::write(&kernel_path, kernel_source).unwrap();
        
        // Compile kernel to PTX
        // On Windows, nvcc requires cl.exe (Visual Studio C++ compiler) to be in PATH
        let ptx_path = PathBuf::from(&out_dir).join("kernel.ptx");
        let nvcc_cmd = nvcc_path.as_ref().map(|s| s.as_str()).unwrap_or("nvcc");
        
        // Find Visual Studio vcvars64.bat to set up the environment
        let vcvars64_path = find_vcvars64();
        
        let compile_result = if let Some(ref vcvars_path) = vcvars64_path {
            // On Windows, we need to run nvcc through cmd.exe with vcvars64.bat
            // Create a batch script that sets up the environment and runs nvcc
            let batch_script_path = PathBuf::from(&out_dir).join("compile_kernel.bat");
            let kernel_path_str = kernel_path.to_string_lossy().replace('/', "\\");
            let ptx_path_str = ptx_path.to_string_lossy().replace('/', "\\");
            let nvcc_cmd_str = nvcc_cmd.replace('/', "\\");
            let vcvars_path_str = vcvars_path.replace('/', "\\");
            
            // Create batch script that calls vcvars64.bat and then nvcc
            let batch_content = format!(
                "@echo off\n\
                call \"{}\" >nul 2>&1\n\
                \"{}\" -ptx -arch=sm_86 -o \"{}\" \"{}\"\n",
                vcvars_path_str, nvcc_cmd_str, ptx_path_str, kernel_path_str
            );
            
            // Write batch script, if it fails, try direct compilation
            match std::fs::write(&batch_script_path, batch_content) {
                Ok(_) => {
                    // Execute the batch script
                    Command::new("cmd.exe")
                        .arg("/c")
                        .arg(&batch_script_path)
                        .output()
                }
                Err(e) => {
                    println!("cargo:warning=Failed to create batch script: {}", e);
                    println!("cargo:warning=Attempting direct compilation without Visual Studio environment.");
                    // Fallback to direct compilation
                    Command::new(nvcc_cmd)
                        .arg("-ptx")
                        .arg("-arch=sm_86")
                        .arg("-o")
                        .arg(&ptx_path)
                        .arg(&kernel_path)
                        .output()
                }
            }
        } else {
            // Try without Visual Studio environment (might work if cl.exe is already in PATH)
            println!("cargo:warning=Visual Studio vcvars64.bat not found. Attempting compilation without it.");
            Command::new(nvcc_cmd)
                .arg("-ptx")
                .arg("-arch=sm_86") // RTX 3080 Ti has compute capability 8.6
                .arg("-o")
                .arg(&ptx_path)
                .arg(&kernel_path)
                .output()
        };
        
        match compile_result {
            Ok(output) => {
                if output.status.success() && ptx_path.exists() {
                    println!("cargo:warning=CUDA kernel compiled successfully to {:?}", ptx_path);
                    true
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("cargo:warning=Failed to compile CUDA kernel:");
                    if !stderr.is_empty() {
                        println!("cargo:warning=STDERR: {}", stderr);
                    }
                    if !stdout.is_empty() {
                        println!("cargo:warning=STDOUT: {}", stdout);
                    }
                    println!("cargo:warning=Exit code: {:?}", output.status.code());
                    println!("cargo:warning=GPU support will be limited.");
                    false
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run nvcc: {}", e);
                println!("cargo:warning=GPU support will be limited.");
                false
            }
        }
    } else {
        println!("cargo:warning=CUDA Toolkit (nvcc) not found. GPU support will be limited.");
        println!("cargo:warning=Install CUDA Toolkit from https://developer.nvidia.com/cuda-downloads");
        false
    };
    
    // Always generate the module file, even if PTX is not available
    // This ensures the Rust code compiles regardless of CUDA availability
    let module_content = if ptx_available {
        format!(
            "// Auto-generated module with embedded CUDA kernel PTX\n\
            // This file is generated by build.rs\n\
            \n\
            pub const KERNEL_PTX: &str = include_str!(concat!(env!(\"OUT_DIR\"), \"/kernel.ptx\"));\n"
        )
    } else {
        format!(
            "// Auto-generated module with embedded CUDA kernel PTX\n\
            // This file is generated by build.rs\n\
            // NOTE: CUDA kernel PTX is not available. GPU support will not work.\n\
            \n\
            pub const KERNEL_PTX: &str = \"\";\n"
        )
    };
    
    if let Err(e) = std::fs::write(&module_path, module_content) {
        println!("cargo:warning=Failed to write PTX module: {}", e);
    } else {
        println!("cargo:warning=PTX module generated at {:?}", module_path);
    }
}

