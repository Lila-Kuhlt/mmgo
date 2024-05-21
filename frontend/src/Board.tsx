type LineProps = {
    x: number,
    y: number,
    len: number,
    dir: 'h' | 'v'
}

function Line({ x, y, len, dir }: LineProps) {
    return <path strokeWidth=".2" fill="none" stroke="#000"
        d={`m ${x}, ${y} ${dir} ${len}`}
    />
}

type Player = {
    x: number,
    y: number,
    color: string
}

type BoardProps = {
    size: number,
    players?: Player[]
}

export function Board(props: React.PropsWithChildren<BoardProps>) {
    const size = props.size;
    const pad = 90 / size;

    return <svg xmlns="http://www.w3.org/2000/svg" width="960" height="960" viewBox="0 0 96 96">
        <rect width="96" height="96" fill="#DCB35C" />

        {[...new Array(size + 1)].map((_, i) => <Line x={3} y={i * pad + 3} len={90} dir="h" key={i} />)}
        {[...new Array(size + 1)].map((_, i) => <Line x={i * pad + 3} y={3} len={90} dir="v" key={i} />)}

        {props.players?.map(player => <circle
            key={player.y * size + player.x}
            cx={player.x * pad + 3}
            cy={player.y * pad + 3}
            fill={player.color}
            r={40 / size}
        />)}

    </svg>
}
