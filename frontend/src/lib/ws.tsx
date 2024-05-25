import { createContext, useState, useEffect, PropsWithChildren } from 'react';

/**
 * WebSocket Disconnect Codes
 * @see https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code
 */
const WEBSOCKET_CODES = {
    CLOSE_NORMAL: 1000,
    CLOSE_GOING_AWAY: 1001,
    CLOSE_ABNORMAL: 1006,
    SERVER_ERROR: 1011,
    SERVICE_RESTART: 1012,
};

export type MsgHandler = (msg: string[]) => void;

export class WebSocketHandler {

    private handlers: Record<string, MsgHandler> = {}
    private ws?: WebSocket;
    private addr: string;
    private reconnectHandle?: number;
    private reconnectInterval: number = 1200;

    constructor(ws: string) {
        this.addr = ws;
        this.connect();
    }

    public isOpen() {
        return this.ws?.readyState === WebSocket.OPEN
    }

    private connect() {
        this.ws = new WebSocket(this.addr);
        this.ws.onopen = this.onOpen.bind(this);
        this.ws.onclose = this.onClose.bind(this);
        this.ws.onmessage = this.onMessage.bind(this);
    }


    public disconnect() {
        this.ws?.close();
    }

    public registerHandler(what: string, handler: MsgHandler) {
        this.handlers[what] = handler;
    }

    private onMessage(message: MessageEvent) {
        if (typeof message.data !== "string") return;
        const msg = message.data.split(' ');
        this.handlers[msg[0].toUpperCase()]?.(msg.slice(1));
    }

    private onOpen() {
        clearInterval(this.reconnectHandle)
    }

    private onClose() {
        if (this.reconnectHandle === undefined) {
            this.connect()
            return;
        }

        this.reconnectHandle = setInterval(() => this.connect(), this.reconnectInterval);
    }
}

export const WebSocketContext = createContext<WebSocketHandler | null>(null);

export function WebSocketProvider(props: PropsWithChildren<{ url: string }>) {
    const [ws, setWs] = useState<WebSocketHandler | null>(null);

    useEffect(() => {
        const ws = new WebSocketHandler(props.url);
        setWs(ws);
        return () => ws.disconnect();
    }, [props.url]);

    return <WebSocketContext.Provider value={ws}>
        {props.children}
    </WebSocketContext.Provider>
};
