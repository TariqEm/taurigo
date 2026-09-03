import "@taurigo/ui/globals.css";

import { ThemeProvider } from "@taurigo/ui";
import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "@/app/App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
