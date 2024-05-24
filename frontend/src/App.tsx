import { Board } from "Board"
import { GameStateContext, GameStateExt, GameStateProvider, parseMsg } from "lib/game";
import { WebSocketProvider, } from "lib/ws";
import { QRCodeSVG } from "qrcode.react"
import { useContext, useEffect, useState } from "react";

export function App() {

    function updateState(msg: string, gameStateExt: GameStateExt | null) {
        return gameStateExt?.setState((state) => {
            return ({
                board: parseMsg(msg),
                turn: state.turn + 1
            })
        });
    }

    return (<GameStateProvider>

        <GameStateContext.Consumer>
            {gameState =>
                <WebSocketProvider url="ws://localhost:1213" onMsg={(msg) => updateState(msg, gameState)}>

                    <div className="flex justify-center items-center w-dvw h-dvh">
                        <div className="flex flex-row h-dvh py-10">
                            <BoardWrapper />
                            <Sidebar />
                        </div >
                    </div>

                </WebSocketProvider>
            }
        </GameStateContext.Consumer>
    </GameStateProvider>)

}

function BoardWrapper() {
    const gameState = useContext(GameStateContext);

    const width = gameState?.board.width || 3;
    const height = gameState?.board.height || 3;

    return <Board width={width} height={height} board={gameState?.board.board || undefined} />
}


function Sidebar() {
    const gameState = useContext(GameStateContext);
    const [start] = useState(new Date());
    const [time, setTime] = useState("00:00");

    useEffect(() => {
        const timeout = setInterval(() => {
            const diff = new Date().getTime() - start.getTime();
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
            <hr className="bg-black h-[3px] py-2 mb-4" />
            <a href="https://github.com/Lila-Kuhlt/mmgo" rel="noreferrer" target="_blank" >
                <QRCodeSVG value="https://github.com/Lila-Kuhlt/mmgo" />
            </a>
            <hr className="bg-black h-[3px] py-2 my-4" />
        </div>
    </div>
}
