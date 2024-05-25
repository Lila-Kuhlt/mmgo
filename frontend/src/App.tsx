import { Board } from "Board"
import { GameStateContext, GameStateProvider, parseMsg } from "lib/game";
import { WebSocketContext, WebSocketProvider, } from "lib/ws";
import { QRCodeSVG } from "qrcode.react"
import { useContext, useEffect, useState } from "react";

export function App() {
    return <GameStateProvider>
        <WebSocketProvider url="ws://localhost:1213">

            <div className="flex justify-center items-center w-dvw h-dvh">
                <div className="flex flex-row h-dvh py-10">
                    <Game />
                    <Sidebar />
                </div >
            </div>

        </WebSocketProvider>
    </GameStateProvider>
}

function ConnectionUnavailable() {
    return <div className="flex flex-col jusify-center items-center">
        <h1 className="text-xl text-gray-500">Waiting for the Server to start</h1>
    </div>
}

function Game() {
    const gameState = useContext(GameStateContext);
    const websocket = useContext(WebSocketContext);

    websocket?.registerHandler("BOARD", (msg) => gameState?.setState((state) => ({
        board: parseMsg(msg),
        turn: state.turn + 1
    })));

    const width = gameState?.board.width || 3;
    const height = gameState?.board.height || 3;

    if (!websocket?.isOpen()) return <ConnectionUnavailable />

    return <Board width={width} height={height} board={gameState?.board.board || undefined} />
}


function Sidebar() {
    const ws = useContext(WebSocketContext);
    const gameState = useContext(GameStateContext);
    const [time, setTime] = useState("00:00");

    useEffect(() => {
        const timeout = setInterval(() => {
            const diff = new Date().getTime() - (gameState?.board.start ?? new Date()).getTime();
            const mins = Math.floor(diff / 60000);
            const secs = Math.floor(diff / 1000) % 60;
            setTime(mins.toString().padStart(2, '0') + ':' + secs.toString().padStart(2, '0'));
        });

        return () => clearInterval(timeout);
    })

    return <div className="px-5 flex mt-4">
        <div className="h-full whitespace-nowrap font-mono font-bold text-xl flex flex-col leading-5">
            <span>TURN: {gameState?.turn || 0}</span>
            <span>TIME: {time}</span>
            <span>SIZE: {gameState?.board.width || 3}x{gameState?.board.height || 3}</span>
            <span>SERVER: {ws?.isOpen() ? "CONNECTED" : "LOST"}</span>
            <hr className="bg-black h-[3px] py-2 mb-4" />
            <a href="https://github.com/Lila-Kuhlt/mmgo" rel="noreferrer" target="_blank" className="w-fill flex justify-center">
                <QRCodeSVG value="https://github.com/Lila-Kuhlt/mmgo" size={190}/>
            </a>
        </div>
    </div>
}
