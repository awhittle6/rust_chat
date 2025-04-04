import { useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface ChatMessage {
  from: string;
  message: string;
  timestamp: number;
}

export default function ChatRoom() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [messageInput, setMessageInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom when messages update
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Set up event listeners for new messages
  useEffect(() => {
    const setupListeners = async () => {
      // Listen for new messages from the server
      const unlisten1 = await listen<ChatMessage>('new-message', (event) => {
        setMessages(prev => [...prev, event.payload]);
      });

      // Listen for connection closed events
      const unlisten2 = await listen('connection-closed', () => {
        // Handle disconnection
        console.log('Disconnected from chat server');
      });

      // Start listening for messages
      await invoke('listen_for_messages');

      return () => {
        unlisten1();
        unlisten2();
      };
    };

    const unlistenPromise = setupListeners();
    
    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  const sendMessage = async () => {
    if (!messageInput.trim()) return;
    
    try {
      await invoke('send_message', { message: messageInput });
      
      // Add our own message to the UI immediately for better UX
      // The server will echo it back for other participants
      const myMessage: ChatMessage = {
        from: 'Me',
        message: messageInput,
        timestamp: Date.now() / 1000,
      };
      setMessages(prev => [...prev, myMessage]);
      setMessageInput('');
    } catch (error) {
      console.error('Failed to send message:', error);
    }
  };

  const formatTime = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleTimeString([], { 
      hour: '2-digit', 
      minute: '2-digit'
    });
  };

  return (
    <div className="flex flex-col h-screen">
      <div className="p-4 bg-blue-600 text-white">
        <h1 className="text-xl font-bold">Chat Room</h1>
      </div>
      
      <div className="flex-1 overflow-y-auto p-4 bg-gray-100">
        {messages.map((msg, index) => (
          <div 
            key={index} 
            className={`mb-4 max-w-[80%] ${msg.from === 'Me' ? 'ml-auto bg-blue-500 text-white' : 'mr-auto bg-white'} rounded-lg p-3 shadow`}
          >
            <div className="font-bold">{msg.from}</div>
            <div>{msg.message}</div>
            <div className="text-xs text-right mt-1 opacity-75">
              {formatTime(msg.timestamp)}
            </div>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>
      
      <div className="p-4 bg-white border-t flex">
        <input
          type="text"
          value={messageInput}
          onChange={(e) => setMessageInput(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && sendMessage()}
          className="flex-1 p-2 border rounded-l-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Type a message..."
        />
        <button 
          onClick={sendMessage}
          className="px-4 py-2 bg-blue-500 text-white rounded-r-lg hover:bg-blue-600 transition-colors"
        >
          Send
        </button>
      </div>
    </div>
  );
}