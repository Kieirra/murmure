import { useCallback, useEffect, useRef } from 'react';

interface UseAudioCaptureOptions {
    onPcmChunk: (buffer: ArrayBuffer) => void;
}

export const useAudioCapture = ({ onPcmChunk }: UseAudioCaptureOptions) => {
    const audioContextRef = useRef<AudioContext | null>(null);
    const mediaStreamRef = useRef<MediaStream | null>(null);
    const workletNodeRef = useRef<AudioWorkletNode | null>(null);
    const audioReadyRef = useRef(false);
    const onPcmChunkRef = useRef(onPcmChunk);
    onPcmChunkRef.current = onPcmChunk;

    const cleanup = useCallback(() => {
        audioReadyRef.current = false;
        if (workletNodeRef.current) {
            workletNodeRef.current.disconnect();
            workletNodeRef.current = null;
        }
        if (audioContextRef.current) {
            audioContextRef.current.close();
            audioContextRef.current = null;
        }
        if (mediaStreamRef.current) {
            mediaStreamRef.current.getTracks().forEach((track) => track.stop());
            mediaStreamRef.current = null;
        }
    }, []);

    const init = useCallback(async (): Promise<boolean> => {
        if (audioReadyRef.current) {
            await audioContextRef.current?.resume();
            return true;
        }

        if (!navigator.mediaDevices?.getUserMedia) {
            throw new TypeError(
                "Votre navigateur ne supporte pas l'enregistrement audio. Utilisez Chrome ou Firefox recent."
            );
        }

        if (typeof AudioWorkletNode === 'undefined') {
            throw new TypeError(
                "Votre navigateur ne supporte pas l'enregistrement audio. Utilisez Chrome ou Firefox recent."
            );
        }

        const mediaStream = await navigator.mediaDevices.getUserMedia({
            audio: {
                sampleRate: 16000,
                channelCount: 1,
                echoCancellation: true,
                noiseSuppression: true,
            },
        });
        mediaStreamRef.current = mediaStream;

        const audioContext = new AudioContext({ sampleRate: 16000 });
        audioContextRef.current = audioContext;

        const processorCode = `
class PcmProcessor extends AudioWorkletProcessor {
    process(inputs) {
        const input = inputs[0][0];
        if (input) {
            const pcm16 = new Int16Array(input.length);
            for (let i = 0; i < input.length; i++) {
                pcm16[i] = Math.max(-32768, Math.min(32767, Math.round(input[i] * 32767)));
            }
            this.port.postMessage(pcm16.buffer, [pcm16.buffer]);
        }
        return true;
    }
}
registerProcessor("pcm-processor", PcmProcessor);`;

        const blob = new Blob([processorCode], { type: 'application/javascript' });
        const url = URL.createObjectURL(blob);
        await audioContext.audioWorklet.addModule(url);
        URL.revokeObjectURL(url);

        const source = audioContext.createMediaStreamSource(mediaStream);
        const workletNode = new AudioWorkletNode(audioContext, 'pcm-processor');

        workletNode.port.onmessage = (event: MessageEvent) => {
            onPcmChunkRef.current(event.data as ArrayBuffer);
        };

        source.connect(workletNode);
        workletNode.connect(audioContext.destination);
        workletNodeRef.current = workletNode;

        await audioContext.resume();
        audioReadyRef.current = true;
        return true;
    }, []);

    useEffect(() => {
        return () => {
            cleanup();
        };
    }, [cleanup]);

    return { init, cleanup, audioReady: audioReadyRef };
};
