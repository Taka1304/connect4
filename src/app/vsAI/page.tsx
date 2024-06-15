"use client";
import * as wasm_js from "@/../wasm/pkg/wasm.js";
import Board from "@/components/Molecules/Board";
import { Button } from "@/components/ui/button";
import { COLUMNS } from "@/const/board";
import { useEffect, useState } from "react";

const Connect4 = () => {
	const [vsAi, setVsAi] = useState<wasm_js.VsAi | null>(null);
	const [board, setBoard] = useState<wasm_js.BitBoard>();
	const [turn, setTurn] = useState(true);
	const [gameOver, setGameOver] = useState(false);
	const [winner, setWinner] = useState<number | undefined>();

	const initializeGame = () => {
		const g = new wasm_js.VsAi(false); // TODO: 先手後手の選択
		const b = new wasm_js.BitBoard();

		setVsAi(g);
		setBoard(b);
		setGameOver(false);
		setWinner(undefined);
		setTurn(true);
	};

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		if (!vsAi || !board) {
			return;
		}

		const isEnd = board.judge();
		if (isEnd) {
			setGameOver(true);
			setWinner(isEnd);
			return;
		}

		if (!turn) {
			const ai_move = vsAi.choose_action(board);
			board.drop_disc(ai_move, false);
			setBoard(board);
			setTurn(true);
		}
	}, [turn]);

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		fetch("@/../pkg/wasm_bg.wasm") // fetch /public/pkg/wasm_bg.wasm
			.then((response) => {
				return response.arrayBuffer();
			})
			.then((bytes) => {
				wasm_js.initSync(bytes); // initialize the wasm module
				initializeGame();
			})
			.catch((error) => {
				console.error("Error fetching wasm module:", error);
			});
	}, []);

	if (!vsAi || !board) {
		return <div>Loading...</div>;
	}

	const dropDisk = (column: number) => {
		if (gameOver || !turn) {
			return;
		}

		if (board.is_column_full(column)) {
			return;
		}
		console.log("dropDisk", column);

		board.drop_disc(column, true);

		setBoard(board);
		setTurn(false);
	};

	return (
		<div className="flex flex-col items-center">
			<div>
				<h1>Connect4</h1>
				<Board board={board} />
				<div className="w-full flex justify-around">
					{gameOver ||
						Array.from({ length: COLUMNS }).map((_, idx) => (
							<Button
								type="button"
								key={`key${idx << 1}`}
								onClick={() => dropDisk(idx)}
							>
								{idx + 1}
							</Button>
						))}
				</div>
				{gameOver && (
					<div>
						<h2>
							{winner === 0
								? "Draw"
								: winner === 1
									? "Red wins!"
									: "Yellow wins!"}
						</h2>
						<Button onClick={initializeGame}>もう一度</Button>
					</div>
				)}
			</div>
		</div>
	);
};

export default Connect4;
