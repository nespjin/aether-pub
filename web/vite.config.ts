import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), tsconfigPaths()],
  ssr: {
    noExternal: [
      // fix mui-icons for vite ssr
      // https://github.com/mui/material-ui/issues/43980#issuecomment-2404066648
      process.env.NODE_ENV === "development"
        ? "@mui/icons-material"
        : /^@mui\//,
    ],
  },
  server: {
    hmr: {
      overlay: false,
    },
  },
});
