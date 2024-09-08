import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default {
  input: "src/index.ts",
  output: {
    file: "toucan.js",
    format: "esm",
  },
  plugins: [
    resolve(), // so Rollup can find node modules
    commonjs(), // so Rollup can convert node modules to an ES module
  ],
};
