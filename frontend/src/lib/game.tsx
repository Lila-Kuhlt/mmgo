import { PropsWithChildren, createContext, useState } from "react";

export type Player = {
    x: number,
    y: number,
    color: string
}

export interface GameState {
    board: Player[],
    width: number,
    height: number,
    turn: number,
}

export type GameStateExt = GameState & { setState: (state: GameState) => void };

export const GameStateContext = createContext<GameStateExt | null>(null);

export function GameStateProvider(props: PropsWithChildren) {
    const [gameState, setState] = useState(() => ({ width: 3, height: 3, turn: 0, board: parseBoard('.........', 3) }));
    const stateExt: GameStateExt = { ...gameState, setState: (game: GameState) => setState(game) };

    return <GameStateContext.Provider value={stateExt}>
        {props.children}
    </GameStateContext.Provider>
}


export function parseMsg(msg: string): { board: Player[], width: number, height: number } {
    const [_, widthStr, heightStr, pieces] = msg.split(" ");

    const width = parseInt(widthStr)
    const height = parseInt(heightStr)

    const board = parseBoard(pieces, width);

    return { width, height, board }
}

export function parseBoard(encBoard: string, width: number): Player[] {
    return encBoard.split('')
        .map((color, index) => ({
                x: index % width,
                y: Math.ceil(index / width),
                color
            })
        )
        .filter(piece => piece.color !== '.')
}
