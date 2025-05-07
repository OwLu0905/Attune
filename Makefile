.PHONY: run start


start:
	WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev

run:
	WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev --no-watch
