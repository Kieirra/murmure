#!/usr/bin/env python3
"""
Simple HTTP server to block a port for testing error scenarios.
Run this script to occupy a port, then test Murmure's error handling.

Usage:
    python test_port_blocking.py 4800    # Block port 4800
    python test_port_blocking.py 4801    # Block port 4801
"""

import http.server
import socketserver
import sys
import signal

PORT = 4800

if len(sys.argv) > 1:
    try:
        PORT = int(sys.argv[1])
    except ValueError:
        print(f"Invalid port: {sys.argv[1]}")
        sys.exit(1)

class SimpleHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        self.wfile.write(b'Port is blocked for testing\n')

def signal_handler(sig, frame):
    print(f'\nShutting down port {PORT}...')
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

# Force IPv4 only by setting SO_REUSEADDR
socketserver.TCPServer.allow_reuse_address = True

try:
    # Bind explicitly to 127.0.0.1 IPv4 only
    server = socketserver.TCPServer(("127.0.0.1", PORT), SimpleHandler)
    print(f"Blocking port {PORT} on 127.0.0.1 (IPv4)")
    print("Press Ctrl+C to stop")
    server.serve_forever()
except OSError as e:
    print(f"Error: {e}")
    print(f"Port {PORT} might already be in use or permission denied")
    sys.exit(1)
