from fastapi import FastAPI, File, UploadFile, HTTPException, Form
from fastapi.responses import StreamingResponse
import whisperx
import json
import os
import tempfile
import uuid
from typing import Optional, Generator
from pydantic import BaseModel

app = FastAPI(title="WhisperX Transcription API", version="1.0.0")


class TranscriptionResponse(BaseModel):
    segments: list
    output_file: str
    language: str


class StatusUpdate(BaseModel):
    status: str
    message: str
    progress: Optional[float] = None


def get_device_config():
    return "cpu", "int8"


def save_data(filename: str, data):
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def transcribe_audio_with_status(
    *,
    model_size: str,
    model_path: str,
    file_path: str,
    lang: str,
    output: str,
    initial_prompt: Optional[str] = None,
) -> Generator[str, None, tuple]:
    """
    Transcribe audio and yield status updates during the process
    """
    try:
        device, compute_type = get_device_config()

        # Status: Loading model
        yield f"data: {json.dumps({'status': 'loading_model', 'message': f'Loading {model_size} model...', 'progress': 0.1})}\n\n"

        asr_options = {}
        if initial_prompt:
            asr_options["initial_prompt"] = initial_prompt

        print("initial prompt is:", asr_options)
        model = whisperx.load_model(
            model_size,
            device,
            compute_type=compute_type,
            download_root=model_path,
            asr_options=asr_options,
        )

        # Status: Loading audio
        yield f"data: {json.dumps({'status': 'loading_audio', 'message': 'Loading audio file...', 'progress': 0.2})}\n\n"

        audio = whisperx.load_audio(file_path)

        # Status: Transcribing
        yield f"data: {json.dumps({'status': 'transcribing', 'message': 'Transcribing audio...', 'progress': 0.3})}\n\n"

        result = model.transcribe(audio, batch_size=8, print_progress=True)
        language = result["language"]

        # Status: Loading alignment model
        yield f"data: {json.dumps({'status': 'loading_alignment', 'message': 'Loading alignment model...', 'progress': 0.7})}\n\n"

        model_a, metadata = whisperx.load_align_model(
            language_code=result["language"], device=device
        )

        # Status: Aligning
        yield f"data: {json.dumps({'status': 'aligning', 'message': 'Aligning transcription...', 'progress': 0.8})}\n\n"

        result = whisperx.align(
            result["segments"],
            model_a,
            metadata,
            audio,
            device,
            return_char_alignments=False,
        )

        segments_data = result["segments"]
        filename = f"{output}.json"

        # Status: Saving results
        yield f"data: {json.dumps({'status': 'saving', 'message': 'Saving results...', 'progress': 0.9})}\n\n"

        save_data(filename, segments_data)

        # Status: Complete - send final result
        final_result = {
            "status": "complete",
            "message": "Transcription completed successfully!",
            "progress": 1.0,
            "result": {
                "segments": segments_data,
                "output_file": filename,
                "language": language,
            },
        }
        yield f"data: {json.dumps(final_result)}\n\n"

        return segments_data, filename, language

    except Exception as e:
        error_result = {
            "status": "error",
            "message": f"Transcription failed: {str(e)}",
            "progress": None,
        }
        yield f"data: {json.dumps(error_result)}\n\n"
        raise HTTPException(status_code=500, detail=f"Transcription failed: {str(e)}")


@app.post("/transcribe")
async def transcribe_endpoint(
    file: UploadFile = File(...),
    model: Optional[str] = Form("base.en"),
    model_path: Optional[str] = Form("./models"),
    lang: Optional[str] = Form("en"),
    output_dir: Optional[str] = Form("./outputs"),
    initial_prompt: Optional[str] = Form(None),
):
    """
    Transcribe an audio file using WhisperX with streaming status updates
    """

    # Validate file type
    if not file.content_type.startswith("audio/"):
        raise HTTPException(status_code=400, detail="File must be an audio file")

    # Create temporary file for uploaded audio
    temp_file = None
    try:
        # Generate unique filename
        unique_id = str(uuid.uuid4())
        file_extension = os.path.splitext(file.filename)[1] if file.filename else ".wav"

        # Create temporary file
        with tempfile.NamedTemporaryFile(
            delete=False, suffix=file_extension
        ) as temp_file:
            contents = await file.read()
            temp_file.write(contents)
            temp_file_path = temp_file.name

        def generate_status_updates():
            try:
                for status_update in transcribe_audio_with_status(
                    model_size=model,
                    model_path=model_path,
                    file_path=temp_file_path,
                    lang=lang,
                    output=output_dir,
                    initial_prompt=initial_prompt,
                ):
                    yield status_update
            finally:
                # Clean up temporary file
                if temp_file and os.path.exists(temp_file_path):
                    os.unlink(temp_file_path)

        return StreamingResponse(
            generate_status_updates(),
            media_type="text/event-stream",
            headers={
                "Cache-Control": "no-cache",
                "Connection": "keep-alive",
                "Access-Control-Allow-Origin": "*",
            },
        )

    except Exception as e:
        # Clean up on error
        if temp_file and os.path.exists(temp_file_path):
            os.unlink(temp_file_path)
        raise HTTPException(status_code=500, detail=str(e))


# Keep the original non-streaming endpoint for backward compatibility
@app.post("/transcribe-sync", response_model=TranscriptionResponse)
async def transcribe_sync_endpoint(
    file: UploadFile = File(...),
    model: Optional[str] = Form("base.en"),
    model_path: Optional[str] = Form("./models"),
    lang: Optional[str] = Form("en"),
    output_dir: Optional[str] = Form("./outputs"),
    initial_prompt: Optional[str] = Form(None),
):
    """
    Transcribe an audio file using WhisperX (non-streaming version)
    """

    # Validate file type
    if not file.content_type.startswith("audio/"):
        raise HTTPException(status_code=400, detail="File must be an audio file")

    # Create temporary file for uploaded audio
    temp_file = None
    try:
        # Generate unique filename
        unique_id = str(uuid.uuid4())
        file_extension = os.path.splitext(file.filename)[1] if file.filename else ".wav"

        # Create temporary file
        with tempfile.NamedTemporaryFile(
            delete=False, suffix=file_extension
        ) as temp_file:
            contents = await file.read()
            temp_file.write(contents)
            temp_file_path = temp_file.name

        # Transcribe the audio (original non-streaming version)
        device, compute_type = get_device_config()
        asr_options = {}
        if initial_prompt:
            asr_options["initial_prompt"] = initial_prompt

        model_obj = whisperx.load_model(
            model,
            device,
            compute_type=compute_type,
            download_root=model_path,
            asr_options=asr_options,
        )
        audio = whisperx.load_audio(temp_file_path)
        result = model_obj.transcribe(audio, batch_size=8, print_progress=True)

        language = result["language"]
        model_a, metadata = whisperx.load_align_model(
            language_code=result["language"], device=device
        )
        result = whisperx.align(
            result["segments"],
            model_a,
            metadata,
            audio,
            device,
            return_char_alignments=False,
        )

        segments_data = result["segments"]
        filename = f"{output_dir}.json"
        save_data(filename, segments_data)

        return TranscriptionResponse(
            segments=segments_data, output_file=filename, language=language
        )

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        # Clean up temporary file
        if temp_file and os.path.exists(temp_file_path):
            os.unlink(temp_file_path)


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "service": "WhisperX Transcription API"}


@app.get("/")
async def root():
    """Root endpoint with API information"""
    return {
        "message": "WhisperX Transcription API",
        "version": "1.0.0",
        "endpoints": {
            "POST /transcribe": "Transcribe audio files with streaming status",
            "POST /transcribe-sync": "Transcribe audio files (non-streaming)",
            "GET /health": "Health check",
            "GET /docs": "API documentation",
        },
    }


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8081)
