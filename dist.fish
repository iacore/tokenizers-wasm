#!/usr/bin/fish

wasm-pack build
and pnpm i
and pnpm build
and cp pkg/tokenizers_wasm.d.ts output

and begin
    pushd output
    tar --owner=0 --group=0 --touch -cvzf ../release.tar.gz .
    popd
end
