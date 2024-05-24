import { Player } from "lib/game"

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

type BoardProps = {
    width: number,
    height: number,
    board?: Player[]
    backgroundColor?: string,
}

export function Board(props: React.PropsWithChildren<BoardProps>) {

    const padding = 1;
    const border = padding * 2;

    const width = Math.max(props.width, 3) - 1;
    const height = Math.max(props.height, 3) - 1;

    const viewboxHeight = height * 2.5;
    const viewboxWidth = width * 2.5;

    const spaceH = viewboxWidth / width;
    const spaceV = viewboxHeight / height;

    const spaceMin = Math.max(spaceV, spaceH);

    const backgroundColor = props.backgroundColor || "#ffffff";

    const range = (to: number) => [...new Array(to)].map((_, i) => i);

    return <svg
        xmlns="http://www.w3.org/2000/svg"
        height="100%"
        width="100%"
        viewBox={`0 0 ${viewboxWidth + border} ${viewboxHeight + border}`}>

        <rect
            width={viewboxWidth + 2 * padding}
            height={viewboxHeight + 2 * padding}
            fill={backgroundColor}
        />

        {range(height + 1).map(i =>
            <Line
                key={i}
                dir="h"
                x={padding} // offset
                y={i * spaceH + padding}
                len={viewboxWidth}
            />)}

        {range(width + 1).map(i =>
            <Line
                key={i}
                dir="v"
                x={i * spaceV + padding}
                y={padding} // offset
                len={viewboxHeight}
            />)}

        {props.board?.map(p =>
            <circle
                key={p.y * height + p.x}
                cx={p.x * spaceH + padding}
                cy={p.y * spaceV + padding}
                fill={p.color}
                stroke={backgroundColor}
                strokeWidth={.1}
                r={(padding > 0 ? Math.min(spaceMin / 2, padding) : spaceMin / 2)}
            />)}

    </svg>
}
