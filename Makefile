.PHONY: run

- run:
	WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev
