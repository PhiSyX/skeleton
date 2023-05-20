// NOTE: Voir le fichier vite.config.ts pour voir la configuration des aliases
// vers `app:assets:~/` et `~/`.

import "app:assets:~/index.css";

import React from "react";
import ReactDOM from "react-dom/client";

import Home from "~/home";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Home />
  </React.StrictMode>
);
