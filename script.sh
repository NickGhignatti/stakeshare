rm -rf wasm_files
mkdir wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > src/icrc7/icrc7.did || true
mv target/wasm32-unknown-unknown/release/icrc7.wasm wasm_files
gzip wasm_files/icrc7.wasm

cargo build --target wasm32-unknown-unknown --release --package icrc7_backend
candid-extractor target/wasm32-unknown-unknown/release/icrc7_backend.wasm > src/icrc7_backend/icrc7_backend.did || true

cargo build --target wasm32-unknown-unknown --release --package factory
candid-extractor target/wasm32-unknown-unknown/release/factory.wasm > src/factory/factory.did || true

sed -i -E 's/composite_query/query/g' src/icrc7_backend/icrc7_backend.did

# dfx deploy

# cp src/declarations/factory/factory.did.js src/icrc7_frontend/candid/factory.tsx
# cp src/declarations/icrc7_backend/icrc7_backend.did.js src/icrc7_frontend/candid/backend.tsx

# sed 's/composite_query/query/g' src/declarations/icrc7_backend/icrc7_backend.did.js
# sed 's/composite_query/query/g' src/icrc7_frontend/candid/backend.tsx

# dfx deploy