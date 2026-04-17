# LLM Connect Issues

## Ollama 500 Error

A 500 error from Ollama usually means the model is too large for your available memory.

### Diagnosis

```bash
# Check which models are installed
ollama list

# Check if a model is loaded and using GPU
ollama ps

# Test the model directly
ollama run qwen3.5:4b
```

If `ollama ps` shows **0% GPU**, inference runs entirely on CPU and will be very slow.

### Fix: Use a Smaller Model

| Available RAM/VRAM     | Recommended Model |
| ---------------------- | ----------------- |
| 4 GB                   | `qwen3.5:2b`      |
| 8 GB                   | `qwen3.5:4b`      |
| 16+ GB (or 8+ GB VRAM) | `qwen3.5:8b`      |

```bash
ollama pull qwen3.5:4b
```

Then select the new model in Murmure's LLM Connect settings.

## Ollama Not Detected

Murmure may not auto-detect Ollama in some configurations.

**Fix**: Manually set the Ollama URL in LLM Connect settings:

- Local Ollama: `http://localhost:11434`
- Remote Ollama: `http://<server-ip>:11434`

## LLM Output Contains Quotes or Tags

Some models wrap their output in quotes (`"..."`) or include thinking tags (`<think>...</think>`).

**Fix**:

1. Use recommended models: Qwen 3.5, Ministral
2. Add to your system prompt: "Output only the result. No quotes, no thinking, no explanation."

## LLM Response is Very Slow

- **No GPU**: LLM inference on CPU is slow. Consider a smaller model or getting a GPU.
- **Model too large**: If the model doesn't fit in VRAM, it falls back to CPU. Use `ollama ps` to check GPU usage.
- **First request**: The first request after launching is slower (model loading). Subsequent requests are faster.

## Remote Server Connection Issues

For remote Ollama or OpenAI-compatible servers:

1. Verify the server is reachable: `curl http://<server>:<port>/api/tags`
2. Check firewall settings on both machines
3. Ensure the URL in Murmure includes the protocol (`http://` or `https://`)
4. For Ollama, make sure `OLLAMA_HOST=0.0.0.0` is set on the server to allow remote connections

!!! note "Proxy support"
    HTTP proxy for LLM Connect is not yet supported. If you need proxy support in an enterprise environment, please comment on [#286](https://github.com/Kieirra/murmure/issues/286).
