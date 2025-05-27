.PHONY: build_linux build_vad_linux clean


build_linux: 
	pyinstaller --onefile -n whip_v2-x86_64-unknown-linux-gnu --distpath "../../bin/whisper/"  ./main.py

build_vad_linux: 
	pyinstaller --onefile --add-data "./.venv/lib/python3.12/site-packages/faster_whisper/assets:faster_whisper/assets" -n whip_v2-x86_64-unknown-linux-gnu --distpath "../../bin/whisper/"  ./main.py

clean:
	rm -rf build/ dist/ *.spec

