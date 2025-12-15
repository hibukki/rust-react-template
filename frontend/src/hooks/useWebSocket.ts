import { useEffect, useRef, useState, useCallback } from "react";
import type { WsEvent, Profile } from "../types/bindings";

interface UseWebSocketOptions {
  onProfile?: (profile: Profile) => void;
  reconnectInterval?: number;
}

interface UseWebSocketReturn {
  isConnected: boolean;
  error: string | null;
}

function parseProfile(data: unknown): Profile {
  const raw = data as { id: number; user_id: number; display_name: string; bio: string | null; updated_at: string };
  return {
    id: BigInt(raw.id),
    user_id: BigInt(raw.user_id),
    display_name: raw.display_name,
    bio: raw.bio,
    updated_at: raw.updated_at,
  };
}

function parseWsEvent(data: unknown): WsEvent {
  const raw = data as { type: string; data: unknown };
  if (raw.type === "Profile") {
    return { type: "Profile", data: parseProfile(raw.data) };
  }
  throw new Error(`Unknown event type: ${raw.type}`);
}

export function useWebSocket(options: UseWebSocketOptions = {}): UseWebSocketReturn {
  const { onProfile, reconnectInterval = 3000 } = options;
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const shouldReconnectRef = useRef(true);

  const connect = useCallback(() => {
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/api/ws`;

    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onopen = () => {
      setIsConnected(true);
      setError(null);
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        const wsEvent = parseWsEvent(data);

        // Single handler for all profile events (initial state and updates)
        if (wsEvent.type === "Profile" && onProfile) {
          onProfile(wsEvent.data);
        }
      } catch (e) {
        console.error("Failed to parse WebSocket message:", e);
      }
    };

    ws.onerror = () => {
      setError("WebSocket connection error");
    };

    ws.onclose = () => {
      setIsConnected(false);
      wsRef.current = null;
    };
  }, [onProfile]);

  // Handle reconnection separately
  useEffect(() => {
    if (!isConnected && shouldReconnectRef.current && !wsRef.current) {
      reconnectTimeoutRef.current = setTimeout(() => {
        connect();
      }, reconnectInterval);
    }

    return () => {
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
    };
  }, [isConnected, connect, reconnectInterval]);

  // Initial connection
  useEffect(() => {
    connect();

    return () => {
      shouldReconnectRef.current = false;
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [connect]);

  return { isConnected, error };
}
