# Smart Speech Mic

![Smart Speech Mic](../assets/smart-mic.png)

Smart Speech Mic turns any smartphone into a wireless microphone for Murmure. No app installation required on the phone - just scan a QR code.

## How It Works

1. Murmure starts a secure local HTTPS server on your computer
2. A QR code is displayed in the app
3. You scan the QR code with your phone's camera
4. Your phone opens a web page that streams audio to Murmure over WebSocket
5. Murmure uses the phone audio as its microphone input

## Setup

1. Go to **Extensions** > **Smart Speech Mic**
2. Enable Smart Speech Mic
3. A QR code appears on screen
4. Scan it with your phone (both devices must be on the same network)
5. Allow microphone access in your phone's browser
6. Start recording in Murmure as usual

## Phone Views

The phone web interface has three tabs at the top, each offering a different way to use Smart Speech Mic. The active view is remembered between sessions, so the phone re-opens on the last tab you used.

### Remote

The default mode. Your phone acts as a wireless microphone and remote control for Murmure:

- The **REC** button streams your voice to the computer and pastes the transcription into the focused text field
- The **trackpad** controls the mouse pointer (tap to click, long-press for right-click)
- The **Enter** and **Backspace** buttons send keyboard events to the computer

Use this mode for hands-free dictation when the text should appear directly in a computer application.

### Transcription

A chronological log of transcriptions displayed on the phone. Useful when you want to read or share what has been dictated without sending it to a specific application on the computer.

- Each transcription is time-stamped
- Tap any entry to copy its text to the phone clipboard
- Use **Copy all** to copy the full history at once
- The three-dot menu lets you clear the history

### Translation

Bidirectional translation between two languages, displayed as chat bubbles. Pick the language pair at the top of the view, press **REC**, and speak. Each utterance is detected, translated, and displayed in the appropriate bubble side.

- The selected language pair is persisted between sessions
- Messages from each language appear on opposite sides of the conversation
- A pulsing bubble indicates a translation in progress

The phone web interface is available in several languages and follows the language of the phone's browser (you can also force one via the `?lang=` URL parameter).

## Requirements

- Both devices must be on the **same local network** (Wi-Fi)
- Your phone needs a modern browser (Chrome, Safari, Firefox)
- Microphone permission must be granted in the phone browser

## Security

- Communication is encrypted via TLS (HTTPS + WSS)
- Self-signed certificates are generated locally
- Device pairing uses tokens stored in your system's native keyring
- No data leaves your local network

## Managing Paired Devices

You can view and remove paired devices in the Smart Speech Mic settings. Removing a device revokes its pairing token.

## Configuration

- **Port**: The server port can be changed in settings if the default conflicts with another service
- **Bind address**: In **Advanced Settings**, choose which network interface the Smart Mic server listens on. Leave it on **Auto** (default) in most cases, or pick a specific IP to force traffic through a particular interface (VPN, dedicated LAN, etc.)
- **Enable/Disable**: Toggle the Smart Mic server on/off as needed

## Remote Access

By default, Smart Speech Mic works on your local network. For remote access (from a different network, mobile data, etc.), you can configure a relay in **Advanced Settings**.

### Option 1: Network Segmentation (Enterprise)

The simplest approach for hospitals and enterprises. No changes needed in Murmure.

Your IT department creates a dedicated Wi-Fi network for staff with firewall rules allowing traffic only on Murmure's port (default: 4801) toward the workstation network. For example:

```
Staff Wi-Fi (10.0.1.0/24)          Private Network (192.168.1.0/24)
   Phone (10.0.1.50)    ------>    Workstation (192.168.1.100:4801)
                         Firewall rule:
                         ALLOW 10.0.1.0/24 -> 192.168.1.0/24 port 4801
                         DENY everything else
```

The phone connects to the staff Wi-Fi, scans the QR code, and connects directly to the workstation. An attacker would need to be physically present, authenticated on the staff Wi-Fi, and exploit a vulnerability on Murmure's specific port.

### Option 2: Reverse Proxy with SSO (Enterprise)

For organizations that cannot put staff phones on a network segment with access to workstations, a reverse proxy (Nginx, Caddy) can route traffic from an external-facing endpoint to internal Murmure instances.

**In Murmure:**

1. Set **Relay URL** to your proxy address (e.g., `https://smartmic.hospital.com`)
2. Enable **Machine ID** to include a machine identifier in the URL
3. The QR code will encode: `https://smartmic.hospital.com/pc-urgences-01/?token=...`

!!! warning
    Machine ID adds a path prefix to the URL. The reverse proxy must strip this prefix before forwarding the request to the Murmure server, otherwise routes (`/`, `/ws`, etc.) will return a 404 error. A simple Cloudflare Tunnel (Option 3) cannot perform this routing, use Nginx or Caddy with a path rewrite rule.

**On the IT side**, a conceptual Nginx configuration:

```nginx
server {
    listen 443 ssl;
    server_name smartmic.hospital.com;

    # oauth2-proxy handles Keycloak authentication
    location /oauth2/ {
        proxy_pass http://127.0.0.1:4180;
    }

    # Route /{machine-id}/* to the corresponding workstation
    location ~ ^/(?<machine>[^/?]+)(?<rest>/.*)?$ {
        auth_request /oauth2/auth;
        error_page 401 = /oauth2/sign_in?rd=$scheme://$host$request_uri;

        proxy_pass https://$machine.internal.hospital.com:4801$rest;
        proxy_ssl_verify off;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

The proxy resolves the machine name via internal DNS (e.g., `pc-urgences-01.internal.hospital.com`), forwards WebSocket traffic, and handles SSO authentication. Staff phones connect from any network (guest Wi-Fi, 4G) without accessing the internal network directly.

### Option 3: Cloud Tunnel (Personal)

For personal use, [Cloudflare Tunnel](https://developers.cloudflare.com/tunnel/) is the simplest option. It creates a secure outbound connection from your computer to Cloudflare's network, giving you a public URL without opening any ports.

**Step 1: Install `cloudflared`**

Download the binary for your platform from the [official Cloudflare downloads page](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/).

**Step 2: Start the tunnel**

```bash
cloudflared tunnel --url https://localhost:4801 --no-tls-verify
```

The `--no-tls-verify` flag is required because Murmure uses a self-signed certificate for its local server.

You will see output like:

```
+--------------------------------------------------------------------------------------------+
|  Your quick Tunnel has been created! Visit it at (it may take some time to be reachable):  |
|  https://random-name-here.trycloudflare.com                                               |
+--------------------------------------------------------------------------------------------+
```

**Step 3: Configure Murmure**

1. Copy the generated URL (e.g., `https://random-name-here.trycloudflare.com`)
2. Open Murmure > Smart Speech Mic > **Advanced Settings**
3. Paste the URL into the **Relay URL** field
4. Leave **Machine ID** disabled (it is not needed for personal use)
5. Scan the QR code from your phone on any network (4G, another Wi-Fi, etc.)

The tunnel stays active as long as the `cloudflared` command is running. Close it with `Ctrl+C` when you are done.

!!! warning
    When using a cloud tunnel, your audio data transits through the tunnel provider's servers (Cloudflare in this case). For medical or sensitive data, use a self-hosted solution (Option 1 or 2) instead.

### Token Expiration

In Advanced Settings, you can set a **Token expiration** (in hours). When set, paired devices are automatically revoked after the specified duration. Set to 0 for no expiration (default).

This is useful in shared environments (hospitals, shared workstations) where sessions should be time-limited.
