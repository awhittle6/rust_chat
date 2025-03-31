import React, { useState } from 'react'

interface HomeProps {
  startServer: () => void;
  joinServer: (addr: string) => void;
}

const Home : React.FC<HomeProps> = ({ startServer, joinServer }) => {
  const [addr, setAddr] = useState("");
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
        <button type="button" onClick={() => joinServer(addr)}>Join</button>
      </div>
    </div>
  </main>
  )
}

export default Home