import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [addr, setAddr] = useState("");

async function startServer() {
  await invoke("start_server");
}

async function joinServer() {
  await invoke("join_server", { addr });
}
  return (
    <main className="w-screen h-screen flex justify-center items-center">
      <div className="flex flex-col gap-4 w-screen">
        <h1 className="text-3xl font-bold underline">Chatter</h1>
        <div className="flex flex-col gap-2 justify-center items-center ">
          <label htmlFor="server-address">Start Room</label>
          <button type="button" onClick={() => startServer()}>Start</button>
        </div>
        <div className="flex flex-col gap-2 justify-center items-center ">
          <label htmlFor="server-address">Join Room</label>
          <input type="text" id="server-address" onChange={(e) => {e.preventDefault(); setAddr(e.currentTarget.value)}} />
          <button type="button" onClick={() => joinServer()}>Join</button>
        </div>
      </div>



      {/* <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p> */}
    </main>
  );
}

export default App;
