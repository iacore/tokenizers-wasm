import { wasm } from '@rollup/plugin-wasm';
import { defineConfig } from "rollup";

export default defineConfig({
  input: 'pkg/tokenizers_wasm.js',
  output: {
    dir: 'output',
    format: 'esm',
  },
  plugins: [wasm({targetEnv: "browser"})]
});
