import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import ChatRoom from "./components/screens/ChatRoom";
import Home from "./components/screens/Home";

function App() {
  const [addr, setAddr] = useState("");
  const [inRoom, setInRoom] = useState(false);

async function startServer() {
  if (inRoom) {
    return;
  }
  const startedServer : boolean = await invoke("start_server") as unknown as boolean;
  const joinedServer : boolean = await invoke("join_server", { addr: "http://127.0.0.1:50051" }) as unknown as boolean;
  setInRoom(joinedServer);
}

async function joinServer() {
  if (!addr || inRoom) {
    return;
  }
  const joinedServer : boolean = await invoke("join_server", { addr }) as unknown as boolean;
  setInRoom(joinedServer);
}

  if (inRoom){
    return (
      <ChatRoom />
    )
  }
  return (
    <Home startServer={startServer} joinServer={joinServer} />
  );
}

export default App;
