# handy-cli

Transcribe WAV files offline using a model already installed via [Handy](https://handy.computer).

---

## Install

```sh
cargo install --git https://github.com/pascalwengerter/handy-cli
```

---

## Usage

```sh
handy-cli --input audio.wav --model /path/to/model
```

---

## Input Requirements

- **Format:** 16kHz, mono, 16-bit PCM WAV
- Convert other formats:
  ```sh
  ffmpeg -i input.ogg -ar 16000 -ac 1 -c:a pcm_s16le output.wav
  ```

---
## Configuration

Set the model path via:
- CLI: `--model /path/to/model`
- `.env`: `HANDY_MODEL_PATH=/path/to/model`
- Environment variable: `export HANDY_MODEL_PATH=/path/to/model`
