.PHONY: build_linux build_mac build_vad_linux build_vad_mac clean


build_linux: 
	pyinstaller --onefile -n whip_v2-x86_64-unknown-linux-gnu --distpath "../../bin/whisper/"  ./main.py

build_mac: 
	pyinstaller --onefile -n whip_v2-aarch64-apple-darwin --distpath "../../bin/whisper/"  ./main.py

build_vad_linux: 
	pyinstaller --onefile --add-data "./.venv/lib/python3.12/site-packages/faster_whisper/assets:faster_whisper/assets" -n whip_v2-x86_64-unknown-linux-gnu --distpath "../../bin/whisper/"  ./main.py

build_vad_mac: 
	pyinstaller --onefile --add-data "./.venv/lib/python3.12/site-packages/faster_whisper/assets:faster_whisper/assets" -n whip_v2-aarch64-apple-darwin --distpath "../../bin/whisper/"  ./main.py

clean:
	rm -rf build/ dist/ *.spec

