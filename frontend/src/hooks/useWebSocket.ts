import { useState, useEffect } from 'react';

export function useWebSocket(projectId: string | null) {
  const [messages, setMessages] = useState<any[]>([]);
  const [status, setStatus] = useState<'connecting' | 'connected' | 'disconnected'>('disconnected');

  useEffect(() => {
    if (!projectId) return;
    
    // In actual implementation, connect to ws://localhost:3001/ws
    setStatus('connected');
    
    return () => {
      setStatus('disconnected');
    };
  }, [projectId]);

  return { messages, status };
}
