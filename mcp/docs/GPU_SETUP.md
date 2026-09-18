# GPU Acceleration Setup for vim_rs MCP Server

This MCP server supports NVIDIA GPU acceleration via CUDA for significantly faster embedding generation and semantic search.

## Performance Impact

Query works decently with or without CUDA. Building the API database without CUDA is a multi-minute
wait. With CUDA it is about half a minute on a good machine.

## Requirements

### Hardware
- NVIDIA GPU with Compute Capability 7.0+ (Maxwell architecture or newer)
- Recommended: RTX 3000/4000 series or higher
- Tested on: RTX 4090

### Software Dependencies

You need the **CUDA runtime DLLs** (not the full SDK), specifically:

1. **CUDA 12.x Runtime**
   - Download from: https://developer.nvidia.com/cuda-downloads
   - You can install the full CUDA Toolkit or just the runtime redistributables
   - **Windows**: Adds to `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin`
   - **Linux**: Installs to `/usr/local/cuda-12.x/lib64`

2. **cuDNN 9.x**
   - Download from: https://developer.nvidia.com/cudnn (requires free NVIDIA Developer account)
   - Extract and add the `bin` (Windows) or `lib` (Linux) directory to your PATH

### Environment Setup

**Windows:**
```powershell
# Add CUDA and cuDNN to PATH
$env:PATH += ";C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin"
$env:PATH += ";C:\path\to\cudnn\bin"
```

**Linux:**
```bash
# Add to ~/.bashrc or ~/.zshrc
export LD_LIBRARY_PATH=/usr/local/cuda-12.x/lib64:$LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/path/to/cudnn/lib:$LD_LIBRARY_PATH
```

## Building with CUDA Support

### MCP Server

```bash
cd mcp
cargo build --release --features cuda
```

### Data Transformer (embedding build)

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
```

## Verification

When you run with CUDA enabled, you should see:

```
[INFO] CUDA feature enabled - using GPU acceleration
[INFO] Successfully registered `CUDAExecutionProvider`
[INFO] Model initialized successfully
```

If CUDA is not available (missing DLLs, incompatible GPU), the system will gracefully fallback to CPU with a warning.

## Troubleshooting

### Quick checks

1. Verify GPU driver: `nvidia-smi`
2. Verify CUDA toolkit (optional): `nvcc --version`
3. Confirm CUDA/cuDNN runtime DLLs are on `PATH`, or copied next to the binary in `mcp/target/release/`
4. On Windows, cuDNN 9.x is often installed under `C:\Program Files\NVIDIA\CUDNN\v9.x\bin\12.9\` (pick the CUDA major version that matches your toolkit)

### Error 1114: "A dynamic link library (DLL) initialization routine failed"

This message usually means `onnxruntime_providers_cuda.dll` **was found** but failed to initialize — not that a dependency is missing. Common causes:

1. **Stale ONNX Runtime provider DLLs** in `mcp/target/release/` after upgrading the `ort` crate. The `copy-dylibs` feature copies provider DLLs at build time but **does not overwrite** files that already exist, so an old `onnxruntime_providers_cuda.dll` can linger and mismatch the statically linked ONNX Runtime core.
2. **A running process has the DLL locked** (e.g. `vim_mcp_server.exe`). Stop it before replacing DLLs.
3. **CUDA/cuDNN version skew** — the `ort` prebuilt binaries target CUDA 12.x (`cu12`) by default; ensure `cudart64_12.dll` and `cudnn64_9.dll` match that stack.

**How to tell if provider DLLs are stale (Windows):**

Compare the file in `target/release` with the copy in the ort download cache. They should be the same size:

```powershell
$release = "mcp\target\release\onnxruntime_providers_cuda.dll"
$cache   = "$env:LOCALAPPDATA\ort.pyke.io\dfbin\x86_64-pc-windows-msvc\8a54165e2dfc85e9f6afbdaf154e7c1c74582e6269a2d0ec93b11e1459309555\onnxruntime_providers_cuda.dll"
(Get-Item $release).Length
(Get-Item $cache).Length   # should match; hash dir name changes when ort version/feature set changes
```

The cache path is under `%LOCALAPPDATA%\ort.pyke.io\dfbin\<target>\<hash>\` (not `%LOCALAPPDATA%\ort`). List available builds:

```powershell
Get-ChildItem "$env:LOCALAPPDATA\ort.pyke.io\dfbin\x86_64-pc-windows-msvc" -Directory
```

**Refresh provider DLLs (Windows):**

```powershell
Get-Process vim_mcp_server, data-transformer -ErrorAction SilentlyContinue | Stop-Process -Force

$release = "mcp\target\release"
$cache   = "$env:LOCALAPPDATA\ort.pyke.io\dfbin\x86_64-pc-windows-msvc\8a54165e2dfc85e9f6afbdaf154e7c1c74582e6269a2d0ec93b11e1459309555"

Remove-Item "$release\onnxruntime_providers*.dll" -Force
Copy-Item "$cache\onnxruntime_providers*.dll" $release -Force
```

Or force a clean copy via rebuild:

```bash
cd mcp
cargo clean -p data-transformer
cargo run -p data-transformer --release --features cuda
```

**CUDA/cuDNN runtime DLLs** still need to be reachable. If not on `PATH`, copy these next to the binary:

- From CUDA 12.x `bin`: `cudart64_12.dll`, `cublas64_12.dll`, `cublasLt64_12.dll`, `cufft64_11.dll`
- From cuDNN 9.x `bin\12.9`: all `cudnn*.dll` files

### "CUDA provider is not available" / falls back to CPU

1. Verify CUDA installation: `nvcc --version` (if you installed full toolkit)
2. Check GPU driver: `nvidia-smi`
3. Ensure CUDA bin directory is on PATH
4. Ensure cuDNN bin directory is on PATH
5. See **Error 1114** above if logs mention DLL initialization failure

### "Cannot find cudnn64_9.dll" (Windows)

- Download cuDNN 9.x from NVIDIA
- Extract and add `bin` directory to PATH, or copy DLLs into `mcp/target/release/`

### "libcudnn.so.9: cannot open shared object file" (Linux)

- Install cuDNN 9.x
- Add to LD_LIBRARY_PATH

## Building without CUDA (CPU-only)

Default build uses CPU:

```bash
cargo build --release
```

## Performance Tips

1. **First run downloads model**: Even with GPU, first run downloads ~90MB model (cached for future runs)
2. **Batch operations**: GPU acceleration is most beneficial for batch embedding generation
3. **Memory**: Ensure sufficient GPU memory (model uses ~400MB VRAM)

## Technical Details

- Uses ONNX Runtime with CUDA Execution Provider
- Falls back gracefully to CPU if CUDA unavailable
- Model: BGE-small-en-v1.5 (384-dimensional embeddings)
- CUDA Compute Capability: 7.0+ required
