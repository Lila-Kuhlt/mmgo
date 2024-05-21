import { Board } from "Board"

const PLAYERS = [
    {
        x: 1,
        y: 4,
        color: "red"
    },
    {
        x: 3,
        y: 4,
        color: "white"
    },
];

export function App() {
    return <div className="flex justify-center items-center w-dvw h-dvh">
        <div className="flex flex-row">
            <Board size={20} players={PLAYERS} />
            <div className="px-10">
                <div className="h-full font-mono font-bold text-xl flex flex-col leading-5">
                    <span>TURN: 2</span>
                    <span>SIZE: 20</span>
                    <span>ROUND: 20</span>
                    <span>PLAYERS: 20</span>
                    <hr className="bg-black h-[3px] py-2 mb-8"/>
                </div>
            </div>
        </div>
    </div>
}