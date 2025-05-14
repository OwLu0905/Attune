.PHONY: run start build_linux


start:
	WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev

run:
	WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev --no-watch

build_linux: 
	NO_STRIP=true pnpm tauri build

