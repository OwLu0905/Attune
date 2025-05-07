.PHONY: build_linux clean


build_linux: 
	pyinstaller --onefile -n whip_v2-x86_64-unknown-linux-gnu --distpath "../../bin/whisper/"  ./main.py

clean:
	rm -rf build/ dist/ *.spec

