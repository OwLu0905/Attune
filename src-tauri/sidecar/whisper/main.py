import os
from faster_whisper import WhisperModel
import ctranslate2

import argparse
import json

from faster_whisper.transcribe import Iterable, Segment


def get_device_config():
    # TODO: check if nvcc is available
    # if ctranslate2.get_cuda_device_count() > 0:
    #     return "cuda", "float16"
    # else:
    #     return "cpu", "int8"
    return "cpu", "int8"


def handle_segments(segments: Iterable[Segment]):
    segments_data = []

    for segment in segments:
        # Convert segment object to dictionary
        segment_dict = {
            "start": segment.start,
            "end": segment.end,
            "text": segment.text,
            "words": [],
        }

        # Add word-level timestamps if available
        if segment.words:
            for word in segment.words:
                word_dict = {
                    "word": word.word,
                    "start": word.start,
                    "end": word.end,
                    "probability": word.probability,
                }
                segment_dict["words"].append(word_dict)

        segments_data.append(segment_dict)

    return segments_data


def save_data(filename: str, data):
    directory = os.path.dirname(filename)
    if directory and not os.path.exists(directory):
        os.makedirs(directory)

    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def transcribe(
    *, model_size: str, model_path: str, file_path: str, lang: str, output: str
):
    device, compute_type = get_device_config()

    model = WhisperModel(
        model_size, device=device, compute_type=compute_type, download_root=model_path
    )

    segments, _ = model.transcribe(
        file_path,
        beam_size=5,
        language=lang,
        word_timestamps=True,
        vad_filter=False,
        log_progress=True,
    )

    segments_data = handle_segments(segments=segments)

    filename = f"{output}.json"

    save_data(filename, segments_data)


def run(
    *,
    file: str,
    model: str | None = "base.en",
    model_path: str,
    lang: str | None = "en",
    output: str,
):
    transcribe(
        file_path=file,
        model_size=model or "base.en",
        model_path=model_path,
        lang=lang or "en",
        output=output,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Transcribe audio with Whisper")
    parser.add_argument("--file", type=str, required=True, help="Path to audio file")

    parser.add_argument(
        "--model", type=str, default="base.en", help="Model for transcribe"
    )

    parser.add_argument(
        "--model_path",
        type=str,
        default="./models",
        help="Path of Model (e.g. base.en)",
    )
    parser.add_argument(
        "--lang", type=str, default="en", help="Language for transcribe"
    )

    parser.add_argument(
        "--output", type=str, required=True, help="Path of the transcribe file"
    )
    args = parser.parse_args()

    run(
        file=args.file,
        model=args.model,
        model_path=args.model_path,
        lang=args.lang,
        output=args.output,
    )
