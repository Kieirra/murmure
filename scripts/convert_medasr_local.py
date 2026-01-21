"""
Direct ONNX export for MedASR (lasr_ctc) using torch.onnx.export
Bypasses optimum which is incompatible with transformers 5.0.0.dev0
"""
import torch
import os
from pathlib import Path
from transformers import AutoModelForCTC, AutoProcessor

def export_medasr_to_onnx():
    local_model_path = Path("resources/medasr-native")
    output_dir = Path("resources/medasr-onnx-local")
    output_dir.mkdir(parents=True, exist_ok=True)

    print(f"Loading model from {local_model_path}...")
    
    # Load the processor and model
    processor = AutoProcessor.from_pretrained(local_model_path, trust_remote_code=True)
    model = AutoModelForCTC.from_pretrained(local_model_path, trust_remote_code=True)
    model.eval()
    
    print(f"Model loaded. Type: {type(model)}")
    print(f"Config: {model.config}")

    # Create dummy input (batch=1, 16kHz audio, ~5 seconds)
    # MedASR uses 128 mel bins based on config
    sample_rate = 16000
    duration_seconds = 5
    dummy_audio = torch.randn(sample_rate * duration_seconds)
    
    # Process audio to get input features
    inputs = processor(dummy_audio.numpy(), sampling_rate=sample_rate, return_tensors="pt", padding=True)
    
    print(f"Processor output keys: {inputs.keys() if hasattr(inputs, 'keys') else dir(inputs)}")
    
    # Try different possible key names
    input_values = None
    if hasattr(inputs, 'input_features'):
        input_values = inputs.input_features
    elif hasattr(inputs, 'input_values'):
        input_values = inputs.input_values
    elif isinstance(inputs, dict):
        if 'input_features' in inputs:
            input_values = inputs['input_features']
        elif 'input_values' in inputs:
            input_values = inputs['input_values']    
    
    if input_values is None:
        # Just use the raw audio tensor as input if processor doesn't transform
        print("Processor didn't return expected keys, using raw audio...")
        input_values = dummy_audio.unsqueeze(0)  # Add batch dimension
    
    attention_mask = getattr(inputs, 'attention_mask', None)
    if attention_mask is None and isinstance(inputs, dict):
        attention_mask = inputs.get('attention_mask')
    
    print(f"Input shape: {input_values.shape}")
    
    # Export the model
    onnx_path = output_dir / "model.onnx"
    print(f"Exporting to {onnx_path}...")
    
    # Prepare input names and dynamic axes
    input_names = ["input_values"]
    output_names = ["logits"]
    dynamic_axes = {
        "input_values": {0: "batch", 1: "sequence"},
        "logits": {0: "batch", 1: "time"}
    }
    
    dummy_inputs = (input_values,)
    if attention_mask is not None:
        input_names.append("attention_mask")
        dynamic_axes["attention_mask"] = {0: "batch", 1: "sequence"}
        dummy_inputs = (input_values, attention_mask)

    torch.onnx.export(
        model,
        dummy_inputs,
        str(onnx_path),
        input_names=input_names,
        output_names=output_names,
        dynamic_axes=dynamic_axes,
        opset_version=17,
        do_constant_folding=True,
    )
    
    print(f"Model exported to {onnx_path}")
    
    # Save processor
    processor.save_pretrained(output_dir)
    print(f"Processor saved to {output_dir}")
    
    # Verify the model
    print("Verifying ONNX model...")
    import onnx
    onnx_model = onnx.load(str(onnx_path))
    onnx.checker.check_model(onnx_model)
    print("ONNX model is valid!")
    
    # List output files
    print("\nOutput files:")
    for f in output_dir.iterdir():
        size = f.stat().st_size if f.is_file() else 0
        print(f"  {f.name}: {size / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    export_medasr_to_onnx()
