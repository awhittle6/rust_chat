import React from 'react'



interface ChatRoomProps {
    sendMessage: (message: string) => void;
    messages: string[];
}


const ChatRoom : React.FC<ChatRoomProps>= () => {
  return (
    <div>ChatRoom</div>
  )
}

export default ChatRoom