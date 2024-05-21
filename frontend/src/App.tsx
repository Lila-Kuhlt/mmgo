import { Board } from "Board"
import { QRCodeSVG } from "qrcode.react"

const PLAYERS = [
    {
        x: 0,
        y: 0,
        color: "red"
    },
    {
        x: 1,
        y: 2,
        color: "white"
    },
    {
        x: 2,
        y: 2,
        color: "green"
    },
];

export function App() {
    return <div className="flex justify-center items-center w-dvw h-dvh">
        <div className="flex flex-row">
            <Board width={30} height={4} players={PLAYERS} />
            <div className="px-10">
                <div className="h-full font-mono font-bold text-xl flex flex-col leading-5">
                    <span>TURN: 2</span>
                    <span>SIZE: 20</span>
                    <span>ROUND: 20</span>
                    <span>PLAYERS: 20</span>
                    <hr className="bg-black h-[3px] py-2 mb-4" />
                    <a href="https://github.com/Lila-Kuhlt/mmgo" rel="noreferrer" target="_blank" >
                        <QRCodeSVG value="https://github.com/Lila-Kuhlt/mmgo" />
                    </a>
                </div>
            </div>
        </div>
    </div>
}
