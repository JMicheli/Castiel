import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "../node_modules/bulma/css/bulma.min.css";
import "./index.css";
import App from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
