import { spawn, type ChildProcess } from "node:child_process";
import {
    createWriteStream,
    mkdirSync,
    mkdtempSync,
    rmSync,
    type WriteStream,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(__dirname, "..");
const BINARY_PATH = resolve(PROJECT_ROOT, "src-tauri/target/debug/murmure");
const LOG_DIR = resolve(__dirname, "logs");
const BINARY_LOG = resolve(LOG_DIR, "binary.log");

let tauriDriver: ChildProcess | null = null;
let binaryLog: WriteStream | null = null;
let sandboxDir: string | null = null;

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
            // wdio v9 injects webSocketUrl=true for BiDi, which WebKitWebDriver
            // rejects as "Failed to match capabilities".
            "wdio:enforceWebDriverClassic": true,
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
        // Isolate app data so tests never touch the user's production state.
        sandboxDir = mkdtempSync(join(tmpdir(), "murmure-e2e-"));
        mkdirSync(join(sandboxDir, "data"), { recursive: true });
        mkdirSync(join(sandboxDir, "config"), { recursive: true });
        binaryLog?.write(`[harness] sandbox dir=${sandboxDir}\n`);
        // Private session bus, otherwise tauri-plugin-single-instance sees
        // the user's running Murmure and the test binary exits immediately.
        tauriDriver = spawn(
            "dbus-run-session",
            [
                "--",
                "tauri-driver",
                "--native-driver",
                "/usr/bin/WebKitWebDriver",
            ],
            {
                stdio: ["ignore", "pipe", "pipe"],
                env: {
                    ...process.env,
                    XDG_DATA_HOME: join(sandboxDir, "data"),
                    XDG_CONFIG_HOME: join(sandboxDir, "config"),
                },
            },
        );
        binaryLog?.write(`[harness] tauri-driver spawned pid=${tauriDriver.pid}\n`);
        if (tauriDriver.stdout !== null) {
            tauriDriver.stdout.pipe(binaryLog, { end: false });
        }
        if (tauriDriver.stderr !== null) {
            tauriDriver.stderr.pipe(binaryLog, { end: false });
        }
        tauriDriver.on("error", (err: Error) => {
            binaryLog?.write(`[tauri-driver error] ${err.message}\n`);
        });
        tauriDriver.on("exit", (code, signal) => {
            binaryLog?.write(`[harness] tauri-driver exited code=${code} signal=${signal}\n`);
        });
    },

    onComplete: () => {
        if (tauriDriver !== null && !tauriDriver.killed) {
            tauriDriver.kill();
        }
        binaryLog?.end();
        if (sandboxDir !== null) {
            rmSync(sandboxDir, { recursive: true, force: true });
        }
    },
} as const;
