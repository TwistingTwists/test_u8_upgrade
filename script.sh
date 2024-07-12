dfx start --clean --background --host 127.0.0.1:4943

dfx canister create test_u8_upgrade_backend

dfx build

candid-extractor target/wasm32-unknown-unknown/release/test_u8_upgrade_backend.wasm > ./src/test_u8_upgrade_backend/test_u8_upgrade_backend.did

dfx canister install test_u8_upgrade_backend 