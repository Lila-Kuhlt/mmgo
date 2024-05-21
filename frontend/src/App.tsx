import { Board } from "Board"
import { useState } from "react";

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
    const [size, setSize] = useState(10);

    return <h1 className="text-3xl font-bold underline">
        <Board size={size} players={PLAYERS} />
        <input type="range" min="3" max="50" value={size} onChange={(e) => setSize(parseInt(e.target.value))} />
    </h1>
}
