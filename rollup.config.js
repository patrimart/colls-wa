import typescript from "rollup-plugin-typescript2";

export default {
  input: "js/index.ts",
  output: [
    {
      file: "dist/main.cjs.js",
      format: "cjs"
    },
    {
      file: "dist/main.umd.js",
      format: "umd"
    },
    {
      file: "dist/main.es.js",
      format: "es"
    }
  ],
  plugin: [typescript()]
};
