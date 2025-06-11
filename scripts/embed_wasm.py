import base64
from pathlib import Path

WASM_PATH = Path('pkg/incremental_rust_game_bg.wasm')
JS_PATH = Path('pkg/wasm_base64.js')

def main():
    data = WASM_PATH.read_bytes()
    encoded = base64.b64encode(data).decode('ascii')
    JS_PATH.write_text(f"export const wasm_base64 = `{encoded}`;\n")

if __name__ == '__main__':
    main()
