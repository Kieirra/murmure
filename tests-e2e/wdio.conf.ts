import { spawn, type ChildProcess } from "node:child_process";
import { createWriteStream, mkdirSync, type WriteStream } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(__dirname, "..");
const BINARY_PATH = resolve(PROJECT_ROOT, "src-tauri/target/debug/murmure");
const LOG_DIR = resolve(__dirname, "logs");
const BINARY_LOG = resolve(LOG_DIR, "binary.log");

let tauriDriver: ChildProcess | null = null;
let binaryLog: WriteStream | null = null;

// Typed loosely as `any` because wdio v9 splits config into many overlapping
// types (Testrunner, WithRequestedTestrunnerCapabilities, ...) and a strict
// union here adds noise without catching real bugs at the call site.
export const config = {
    runner: "local",
    framework: "mocha",
    specs: ["./specs/**/*.spec.ts"],
    maxInstances: 1,
    capabilities: [
        {
            "tauri:options": {
                application: BINARY_PATH,
            },
            browserName: "wry",
        },
    ],
    logLevel: "info",
    bail: 0,
    waitforTimeout: 30000,
    connectionRetryTimeout: 60000,
    connectionRetryCount: 3,
    reporters: ["spec"],
    mochaOpts: {
        ui: "bdd",
        timeout: 120000,
    },
    hostname: "127.0.0.1",
    port: 4444,
    path: "/",

    onPrepare: () => {
        mkdirSync(LOG_DIR, { recursive: true });
        binaryLog = createWriteStream(BINARY_LOG, { flags: "w" });
        tauriDriver = spawn("tauri-driver", [], {
            stdio: ["ignore", "pipe", "pipe"],
        });
        binaryLog?.write(`[harness] tauri-driver spawned pid=${tauriDriver.pid}\n`);
        if (tauriDriver.stdout) tauriDriver.stdout.pipe(binaryLog, { end: false });
        if (tauriDriver.stderr) tauriDriver.stderr.pipe(binaryLog, { end: false });
        tauriDriver.on("error", (err: Error) => {
            binaryLog?.write(`[tauri-driver error] ${err.message}\n`);
        });
        tauriDriver.on("exit", (code, signal) => {
            binaryLog?.write(`[harness] tauri-driver exited code=${code} signal=${signal}\n`);
        });
    },

    onComplete: () => {
        if (tauriDriver && !tauriDriver.killed) {
            tauriDriver.kill();
        }
        binaryLog?.end();
    },
} as const;
