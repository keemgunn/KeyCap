// React component that connects to WebSocket server
import { useEffect, useState } from 'react';

function App() {
  const [messages, setMessages] = useState<string[]>([]);

  useEffect(() => {
    // Connect to WebSocket server
    const ws = new WebSocket('ws://localhost:8080/ws');

    ws.onopen = () => {
      console.log('WebSocket connected');
      ws.send('Hello from client!');
    }

    // Event listener for receiving messages
    ws.onmessage = (event) => {
      console.log('Received:', event.data);
      setMessages(prev => [...prev, event.data as string]);
    };

    // If there is an error, log it to the console
    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    // If the connection is closed, log it to the console
    ws.onclose = (event) => {
      console.log('WebSocket disconnected:', event.reason);
    };

    // [TEST] send a message to the server every 5 seconds
    // const interval = setInterval(() => {
    //   ws.send('Interval test message from client!');
    // }, 5000);
    // console.log('Interval set', interval);

    return () => {
      if (ws.readyState === WebSocket.OPEN) {
        console.log('Closing WebSocket');
        ws.close();
      }
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
