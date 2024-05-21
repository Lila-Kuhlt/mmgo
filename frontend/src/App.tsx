import { Board } from "Board"
import { QRCodeSVG } from "qrcode.react"
import { useEffect, useState } from "react";

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

const formatNumber = (n: number) => (n = Math.floor(n)) && Math.abs(n) > 9 ? n.toString() : '0' + n

export function App() {
    const [time, setTime] = useState(0);
    const [timerStart, _] = useState(new Date());

    const [[w, h], setDim] = useState([10, 10]);

    useEffect(() => { setInterval(() => setTime((new Date().getTime() - timerStart.getTime()) / 1000), 100); }, [timerStart])


    return <div className="flex justify-center items-center w-dvw h-dvh">
        <div className="flex flex-row h-dvh py-10">
            <Board width={w} height={h} players={PLAYERS} />
            <div className="px-5 flex">
                <div className="h-full whitespace-nowrap font-mono font-bold text-xl flex flex-col leading-5">
                    <span>TURN: 2</span>
                    <span>SIZE: {w}x{h}</span>
                    <span>ROUND: 20</span>
                    <span>PLAYERS: 20</span>
                    <span>RUNNING: {formatNumber(time / 60)}:{formatNumber(time % 60)}</span>
                    <hr className="bg-black h-[3px] py-2 mb-4" />
                    <a href="https://github.com/Lila-Kuhlt/mmgo" rel="noreferrer" target="_blank" >
                        <QRCodeSVG value="https://github.com/Lila-Kuhlt/mmgo" />
                    </a>
                    <hr className="bg-black h-[3px] py-2 my-4" />
                    <input type="range" min="3" value={w} onChange={(e) => setDim([parseInt(e.target.value), h])} />
                    <input type="range" min="3" value={h} onChange={(e) => setDim([w, parseInt(e.target.value)])} />
                </div>
            </div>
        </div>
    </div>
}
