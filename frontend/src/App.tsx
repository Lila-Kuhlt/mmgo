import { Board } from "Board"
import { GameStateContext, GameStateProvider, parseMsg } from "lib/game";
import { WebSocketProvider, } from "lib/ws";
import { QRCodeSVG } from "qrcode.react"
import { useContext } from "react";

export function App() {


    return (<GameStateProvider>

        <GameStateContext.Consumer>
            {gameState =>
                <WebSocketProvider url="ws://localhost:1213" onMsg={(msg) => gameState?.setState({ ...gameState, ...parseMsg(msg) })}>

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

    const width = gameState?.width || 3;
    const height = gameState?.height || 3;

    return <Board width={width} height={height} board={gameState?.board || undefined} />
}


function Sidebar() {
    const gameState = useContext(GameStateContext);

    return <div className="px-5 flex">
        <div className="h-full whitespace-nowrap font-mono font-bold text-xl flex flex-col leading-5">
            <span>TURN: {gameState?.turn || 0}</span>
            <span>SIZE: {gameState?.width || 3}x{gameState?.height || 3}</span>
            <hr className="bg-black h-[3px] py-2 mb-4" />
            <a href="https://github.com/Lila-Kuhlt/mmgo" rel="noreferrer" target="_blank" >
                <QRCodeSVG value="https://github.com/Lila-Kuhlt/mmgo" />
            </a>
            <hr className="bg-black h-[3px] py-2 my-4" />
        </div>
    </div>
}
