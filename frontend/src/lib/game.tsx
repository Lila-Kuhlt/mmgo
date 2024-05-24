import { PropsWithChildren, createContext, useState } from "react";
import { getColorFromChar } from "./colors";

export type Player = {
    x: number,
    y: number,
    color: string
}

export interface GameState {
    board: Board,
    turn: number,
}

export interface Board {
    board: Player[],
    width: number,
    height: number,
    start: Date
}


export type GameStateExt = GameState & { setState: React.Dispatch<React.SetStateAction<GameState>> };

export const GameStateContext = createContext<GameStateExt | null>(null);

export function GameStateProvider(props: PropsWithChildren) {
    const [gameState, setState] = useState<GameState>({
        turn: 0,
        board: { ...parseBoard('.........', 3, 3), start: new Date() },
    });

    const stateExt: GameStateExt = { ...gameState, setState };

    return <GameStateContext.Provider value={stateExt}>
        {props.children}
    </GameStateContext.Provider>
}


export function parseMsg(msg: string): Board {
    const [_, startStr, widthStr, heightStr, pieces] = msg.split(" ");

    const start = new Date(parseInt(startStr));
    const width = parseInt(widthStr);
    const height = parseInt(heightStr);

    return { ...parseBoard(pieces, width, height), start }
}

export function parseBoard(encBoard: string, width: number, height: number): Omit<Board, 'start'> {
    const board = encBoard.split('')
        .map((color, index) => ({
            x: index % width,
            y: height - 1 - Math.floor((index) / width),
            color
        }))
        .filter(piece => piece.color !== '.')
        .map(piece => ({ ...piece, color: getColorFromChar(piece.color) }))

    return { board, width, height }
}
