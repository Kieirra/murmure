import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

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
    specs: [['./specs/**/*.js']],
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
    connectionRetryTimeout: 10000,
    connectionRetryCount: 3,
    framework: 'mocha',
    mochaOpts: {
        ui: 'bdd',
        timeout: 10000,
    },
    beforeTest: async function () {
        await browser.execute(() => window.location.reload());
        await $('body').waitForExist();
    },

    // ensure the rust project is built since we expect this binary to exist for the webdriver sessions
    onPrepare: () => {
        // Cleanup any zombie processes
        spawnSync('pkill', ['murmure']);
        spawnSync('pkill', ['tauri-driver']);
        spawnSync('pkill', ['WebKitWebDriver']);

        // Remove the extra `--` if you're not using npm!
        spawnSync('pnpm', ['run', 'tauri', 'build', '--debug', '--no-bundle'], {
            cwd: path.resolve(__dirname, '..'),
            stdio: 'inherit',
            shell: true,
        });

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

        // wait for tauri-driver to be ready
        const start = Date.now();
        while (Date.now() - start < 5000) {
            try {
                const status = spawnSync('curl', [
                    'http://127.0.0.1:4444/status',
                ]);
                if (status.status === 0) {
                    break;
                }
            } catch (e) {
                // ignore
            }
            spawnSync('sleep', ['0.1']);
        }
    },

    onComplete: () => {
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
