import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { browser } from "@wdio/globals";

const __dirname = dirname(fileURLToPath(import.meta.url));
const WAV_FIXTURE = resolve(__dirname, "../fixtures/sample.wav");

// Approximation, the real assertion is async further down.
const WAV_DURATION_MS = 3000;
const RELEASE_MARGIN_MS = 1000;
const TRANSCRIPTION_TIMEOUT_MS = 60000;
const TAURI_RUNTIME_READY_TIMEOUT_MS = 30000;

interface TauriInternals {
    invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
}

declare global {
    interface Window {
        __TAURI_INTERNALS__?: TauriInternals;
    }
}

interface InvokeEnvelope<T> {
    __ok?: T;
    __error?: string;
}

async function invokeTauri<T>(
    cmd: string,
    args: Record<string, unknown> = {},
): Promise<T> {
    // W3C classic execute does not serialize Promises, so executeAsync with
    // a done callback is the only way to await tauri.invoke in the webview.
    const result = await browser.executeAsync(
        (
            command: string,
            payload: Record<string, unknown>,
            done: (value: unknown) => void,
        ) => {
            const tauri = window.__TAURI_INTERNALS__;
            if (tauri === undefined) {
                done({ __error: "Tauri runtime not ready" });
                return;
            }
            tauri
                .invoke(command, payload)
                .then((value) => done({ __ok: value }))
                .catch((err) => done({ __error: String(err) }));
        },
        cmd,
        args,
    );
    const envelope = result as InvokeEnvelope<T>;
    if (envelope.__error !== undefined) {
        throw new Error(`invoke ${cmd} failed: ${envelope.__error}`);
    }
    return envelope.__ok as T;
}

function normalize(text: string): string {
    return text
        .normalize("NFD")
        .replace(/[̀-ͯ]/g, "")
        .toLowerCase();
}

describe("golden path", () => {
    it("transcribes the injected WAV and exposes it via history", async () => {
        // Default about:blank has origin "null", so Tauri rejects IPC. The
        // custom scheme gives a real origin and re injects __TAURI_INTERNALS__.
        await browser.url("tauri://localhost");

        await browser.waitUntil(
            async () =>
                (await browser.execute(
                    () => window.__TAURI_INTERNALS__ !== undefined,
                )) === true,
            {
                timeout: TAURI_RUNTIME_READY_TIMEOUT_MS,
                timeoutMsg: "Tauri runtime never became available",
            },
        );

        await invokeTauri<void>("__test_set_audio_source", { wavPath: WAV_FIXTURE });
        await invokeTauri<void>("__test_press_record_shortcut");

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
