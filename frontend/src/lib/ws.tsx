import { createContext, useState, useEffect, useRef, PropsWithChildren } from 'react';

export interface WebSocketState {
    ws: WebSocket | null;
    isConnected: boolean;
}

export interface WebsocketProviderProps {
    url: string,
    onMsg: (msg: string) => void
}

export const WebSocketContext = createContext<WebSocketState | null>(null);

export function WebSocketProvider(props: PropsWithChildren<WebsocketProviderProps>) {
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [isConnected, setIsConnected] = useState<boolean>(false);
    const reconnectInterval = useRef<number | null>(null);

    const connect = () => {
        const newWs = new WebSocket(props.url);
        setWs(newWs);
    };

    useEffect(() => {
        if (!ws) return;

        ws.onopen = () => setIsConnected(true);
        ws.onmessage = (event: MessageEvent) => props.onMsg?.(event.data);

        ws.onclose = () => {
            setIsConnected(false);
            reconnectInterval.current && clearTimeout(reconnectInterval.current);
            reconnectInterval.current = setTimeout(connect, 5000);
        };

        return () => {
            ws.close();
            reconnectInterval.current && clearTimeout(reconnectInterval.current);
        };

    }, [ws]);

    useEffect(() => {
        if (!isConnected) {
            connect();
        }
    }, [isConnected]);

    return <WebSocketContext.Provider value={{ ws, isConnected }}>
        {props.children}
    </WebSocketContext.Provider>
};
