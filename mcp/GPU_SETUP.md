# GPU Acceleration Setup for vim_rs MCP Server

This MCP server supports NVIDIA GPU acceleration via CUDA for significantly faster embedding generation and semantic search.

## Performance Impact

- **CPU**: ~2-5 seconds for embedding queries
- **GPU (RTX 4090)**: ~0.1-0.3 seconds for embedding queries
- **Speedup**: 10-20x faster with GPU acceleration

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

### Build Embeddings Tool

```bash
cd mcp/build-embeddings
cargo build --release --features cuda
cargo run --release --features cuda
```

## Verification

When you run with CUDA enabled, you should see:

```
[INFO] CUDA feature enabled - attempting GPU acceleration
[INFO] Embedding model loaded successfully
```

If CUDA is not available (missing DLLs, incompatible GPU), the system will gracefully fallback to CPU with a warning.

## Troubleshooting

### "CUDA provider is not available"

1. Verify CUDA installation: `nvcc --version` (if you installed full toolkit)
2. Check GPU driver: `nvidia-smi`
3. Ensure CUDA bin directory is on PATH
4. Ensure cuDNN bin directory is on PATH

### "Cannot find cudnn64_9.dll" (Windows)

- Download cuDNN 9.x from NVIDIA
- Extract and add `bin` directory to PATH

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
- Model: all-MiniLM-L6-v2 (384-dimensional embeddings)
- CUDA Compute Capability: 7.0+ required
