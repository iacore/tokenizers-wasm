#!/usr/bin/fish

wasm-pack build
and pnpm i
and pnpm build
and cp pkg/tokenizers_wasm.d.ts output
