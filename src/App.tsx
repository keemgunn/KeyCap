// React component that connects to WebSocket server
import { useEffect, useState } from 'react';

function App() {
  const [messages, setMessages] = useState<string[]>([]);

  useEffect(() => {
    // Connect to WebSocket server
    const ws = new WebSocket('ws://localhost:8080/ws');

    // Event listener for receiving messages
    ws.onmessage = (event) => {
      console.log(event.data);
      setMessages(prev => [...prev, event.data as string]);
    };

    // Clean up function to close socket when component unmounts
    return () => {
      ws.close();
    };
  }, []);

  return (
    <div>
      <h1>Messages</h1>
      <ul>
        {messages.map((msg, index) => (
          <li key={index}>{msg}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;
