import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { browser } from "@wdio/globals";

const __dirname = dirname(fileURLToPath(import.meta.url));
const WAV_FIXTURE = resolve(__dirname, "../fixtures/sample.wav");

// Approximate duration of the committed WAV plus a margin to cover playback
// completion and the VAD flush window. The exact value is not load bearing,
// the assertion is gated by `__test_wait_for_transcription` further down.
const WAV_DURATION_MS = 3000;
const RELEASE_MARGIN_MS = 1000;
const TRANSCRIPTION_TIMEOUT_MS = 60000;

interface TauriCore {
    invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
}

interface TauriWindow extends Window {
    __TAURI__?: { core: TauriCore };
}

async function invokeTauri<T>(
    cmd: string,
    args: Record<string, unknown> = {},
): Promise<T> {
    // wdio v9 awaits any Promise returned from the executed function, so we
    // can simply forward the invoke call.
    const result = await browser.execute(
        (command: string, payload: Record<string, unknown>) => {
            const tauri = (window as unknown as TauriWindow).__TAURI__;
            if (!tauri) {
                throw new Error("Tauri runtime not ready");
            }
            return tauri.core.invoke(command, payload);
        },
        cmd,
        args,
    );
    return result as T;
}

function normalize(text: string): string {
    // NFD decomposes accented chars (è -> e + combining grave), the regex then
    // strips the combining marks (̀-ͯ).
    return text
        .normalize("NFD")
        .replace(/[̀-ͯ]/g, "")
        .toLowerCase();
}

describe("golden path", () => {
    it("transcribes the injected WAV and exposes it via history", async () => {
        // Wait for the webview to expose the Tauri runtime.
        await browser.waitUntil(
            async () =>
                (await browser.execute(
                    () => Boolean((window as unknown as TauriWindow).__TAURI__),
                )) === true,
            { timeout: 30000, timeoutMsg: "Tauri runtime never became available" },
        );

        await invokeTauri<void>("__test_set_audio_source", { wavPath: WAV_FIXTURE });
        await invokeTauri<void>("__test_press_record_shortcut");

        // Let the WAV play through, then release the shortcut so the
        // downstream pipeline runs (VAD flush, transcription, history).
        await browser.pause(WAV_DURATION_MS + RELEASE_MARGIN_MS);
        await invokeTauri<void>("__test_release_record_shortcut");

        const transcript = await invokeTauri<string>(
            "__test_wait_for_transcription",
            { timeoutMs: TRANSCRIPTION_TIMEOUT_MS },
        );

        const normalized = normalize(transcript);
        if (!normalized.includes("murmure")) {
            throw new Error(
                `Transcript does not contain "murmure" after normalization. Got: ${transcript}`,
            );
        }
    });
});
