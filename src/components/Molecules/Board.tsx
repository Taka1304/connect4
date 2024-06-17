import { COLUMNS, ROWS } from "@/const/board";
// components/Board.js
import type React from "react";
import { useEffect, useState } from "react";
import type { BitBoard } from "../../../wasm/pkg/wasm";

type BoardProps = { board: Readonly<BitBoard> };

const Board = ({ board }: BoardProps) => {
	// const [oldBoard, setOldBoard] = useState<BitBoard>(board);

	// TODO: Implement the falling disk animation
	// useEffect(() => {
	//   const diff1 = oldBoard.player1 ^ board.player1;
	//   const diff2 = oldBoard.player2 ^ board.player2;

	//   let newDisk = BigInt(0);

	//   if (diff1) {
	//     newDisk = diff1;
	//   } else if (diff2) {
	//     newDisk = diff2;
	//   }

	//   if (newDisk !== BigInt(0)) {
	//     const diskIndex = newDisk.toString(2).length;
	//     const column = Math.floor(diskIndex / ROWS);
	//     const row = Math.floor((diskIndex) % ROWS);

	//     if (column !== -1 && row !== -1) {
	//       // const disk = { column, row, color: currentPlayer ? 'bg-red-500' : 'bg-yellow-500' };
	//       // setFallingDisks((prev) => [...prev, disk]);

	//       // setTimeout(() => {
	//       //   setFallingDisks((prev) => prev.filter(d => d !== disk));
	//       // }, 1000);
	//     }
	//   }

	//   setOldBoard(board);
	// }, [board.player1, board.player2]);

	return (
		<div className="relative w-[420px] h-[360px] border-2 border-black flex justify-center">
			<div className="grid grid-cols-7 grid-rows-6 gap-1 absolute w-full h-full">
				{Array.from({ length: COLUMNS * ROWS }).map((_, index) => {
					const column = index % COLUMNS;
					const row = ROWS - 1 - Math.floor(index / COLUMNS);
					const isPlayerDisk =
						(board.player1 & (BigInt(1) << BigInt(column * ROWS + row))) !==
						BigInt(0);
					const isAiDisk =
						(board.player2 & (BigInt(1) << BigInt(column * ROWS + row))) !==
						BigInt(0);
					const diskColor = isPlayerDisk
						? "bg-red-500"
						: isAiDisk
							? "bg-yellow-500"
							: "bg-transparent";

					return (
						<div
							key={`static-${index << 1}`}
							className={`w-12 h-12 rounded-full ${diskColor}`}
						/>
					);
				})}
			</div>
		</div>
	);
};

export default Board;
