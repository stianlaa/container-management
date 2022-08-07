import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import path from "path"

export default defineConfig({
    server: {
        port: 5000
    },
    plugins: [svelte()],
    resolve: {
        alias: {
            $utils: path.resolve("./src/utils")
        }
    }
});
