// NOTE: Voir le fichier vite.config.ts pour voir la configuration des aliases
// vers `app:assets:~/`.

import "app:assets:~/home.css";

import reactLogo from "app:assets:~/react.svg"; //
import viteLogo from "/vite.svg";

import { useState } from "react";

function App() {
  const [count, setCount] = useState(0);

  let env = import.meta.env.VITE_APP_HELLO_WORLD;

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React + {env}</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
