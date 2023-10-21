WASM_FLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-arg=--max-memory=4294967296"
CARGO_MODE=""
TARGET_PATH="debug"
BUILD_STD_FEATURES=""

case "$*" in
  *-r*) # -r, --release
    CARGO_MODE="--release"
    TARGET_PATH="release"
    BUILD_STD_FEATURES="panic_immediate_abort"
    ;;
esac

echo "Building with cargo mode: ${CARGO_MODE:-"--debug"}"

OUTPUT_DIR="game"

RUSTFLAGS="${WASM_FLAGS}" \
    cargo +nightly build ${CARGO_MODE} --target wasm32-unknown-unknown \
    -Z "build-std=std,panic_abort" \
    -Z "build-std-features=${BUILD_STD_FEATURES}" && \

wasm-bindgen --out-dir "${OUTPUT_DIR}" \
    --web "target/wasm32-unknown-unknown/${TARGET_PATH}/kart.wasm" && \

wasm-opt \
  -O2 \
  --enable-mutable-globals \
  --enable-bulk-memory \
  --enable-threads \
  --debuginfo \
  "${OUTPUT_DIR}/kart_bg.wasm" \
  -o "${OUTPUT_DIR}/kart_bg.wasm" && \

echo "Done!"