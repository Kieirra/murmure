import fs from 'fs';
import os from 'os';
import path from 'path';
import { spawn, spawnSync } from 'child_process';
import { fileURLToPath } from 'url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const sandboxEnv = createSandboxEnv();
const tauriDriverPath = path.resolve(
    os.homedir(),
    '.cargo',
    'bin',
    'tauri-driver'
);

// keep track of the `tauri-driver` child process
let tauriDriver;
let exit = false;

export const config = {
    host: '127.0.0.1',
    port: 4444,
    specs: ['./specs/**/*.js'],
    services: [
        [
            'visual',
            {
                disableCSSAnimation: false,
                hideScrollBars: false,
                waitForFontsLoaded: false,
                autoElementScroll: false,
                logLevel: 'debug',
            },
        ],
    ],
    maxInstances: 1,
    capabilities: [
        {
            maxInstances: 1,
            'tauri:options': {
                application: '../src-tauri/target/debug/murmure',
            },
        },
    ],
    reporters: ['spec'],
    framework: 'mocha',
    mochaOpts: {
        ui: 'bdd',
        timeout: 60000,
    },

    // ensure the rust project is built since we expect this binary to exist for the webdriver sessions
    onPrepare: () => {
        // Remove the extra `--` if you're not using npm!
        spawnSync(
            'npm',
            ['run', 'tauri', 'build', '--', '--debug', '--no-bundle'],
            {
                cwd: path.resolve(__dirname, '..'),
                stdio: 'inherit',
                shell: true,
            }
        );
    },

    // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests
    beforeSession: () => {
        tauriDriver = spawn(tauriDriverPath, [], {
            env: sandboxEnv,
            stdio: [null, process.stdout, process.stderr],
        });

        tauriDriver.on('error', (error) => {
            console.error('tauri-driver error:', error);
            process.exit(1);
        });
        tauriDriver.on('exit', (code) => {
            if (!exit) {
                console.error('tauri-driver exited with code:', code);
                process.exit(1);
            }
        });
    },

    // clean up the `tauri-driver` process we spawned at the start of the session
    // note that afterSession might not run if the session fails to start, so we also run the cleanup on shutdown
    afterSession: () => {
        closeTauriDriver();
    },
};

function closeTauriDriver() {
    exit = true;
    tauriDriver?.kill('SIGKILL');
}

function onShutdown(fn) {
    const cleanup = () => {
        try {
            fn();
        } finally {
            process.exit();
        }
    };

    process.on('exit', cleanup);
    process.on('SIGINT', cleanup);
    process.on('SIGTERM', cleanup);
    process.on('SIGHUP', cleanup);
    process.on('SIGBREAK', cleanup);
}

// ensure tauri-driver is closed when our test process exits
onShutdown(() => {
    closeTauriDriver();
});

function createSandboxEnv() {
    const root = path.resolve(__dirname, '.tmp', 'wdio-sandbox');
    fs.rmSync(root, { recursive: true, force: true });
    const homeDir = path.join(root, 'home');
    const configDir = path.join(homeDir, '.config');
    const dataDir = path.join(homeDir, '.local', 'share');
    const roamingDir = path.join(homeDir, 'AppData', 'Roaming');
    const localDir = path.join(homeDir, 'AppData', 'Local');
    [configDir, dataDir, roamingDir, localDir].forEach((dir) =>
        fs.mkdirSync(dir, { recursive: true })
    );
    return {
        ...process.env,
        HOME: homeDir,
        USERPROFILE: homeDir,
        XDG_CONFIG_HOME: configDir,
        XDG_DATA_HOME: dataDir,
        APPDATA: roamingDir,
        LOCALAPPDATA: localDir,
    };
}
