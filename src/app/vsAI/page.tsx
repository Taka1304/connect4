"use client";
import * as wasm_js from "@/../wasm/pkg/wasm.js";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";

const Connect4 = () => {
	const [game, setGame] = useState<wasm_js.GameState | null>(null);
	const [board, setBoard] = useState<Uint8Array>(new Uint8Array(42));
	const [gameOver, setGameOver] = useState(false);
	const [winner, setWinner] = useState<wasm_js.Player | null>(null);

	const resetGame = () => {
		const g = wasm_js.GameState.new(1);
		setGame(g);
		setBoard(g.get_board());
		setGameOver(false);
		setWinner(null);
	};

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		fetch("@/../pkg/wasm_bg.wasm") // fetch /public/pkg/wasm_bg.wasm
			.then((response) => {
				return response.arrayBuffer();
			})
			.then((bytes) => {
				wasm_js.initSync(bytes); // initialize the wasm module
				resetGame();
			})
			.catch((error) => {
				console.error("Error fetching wasm module:", error);
			});
	}, []);

	const handleDropDisc = (col: number) => {
		if (!gameOver && game) {
			game.drop_disc(col);
			setBoard(game.get_board());
			const result = game.is_game_over();
			if (result !== undefined) {
				setGameOver(true);
				setWinner(result);
				return;
			}
			// AI move
			game.ai_move();
			setBoard(game.get_board());
			const aiResult = game.is_game_over();
			if (aiResult !== undefined) {
				setGameOver(true);
				setWinner(aiResult);
			}
		}
	};

	const renderBoard = () => {
		const rows = [];
		for (let row = 0; row < 6; row++) {
			const cells = [];
			for (let col = 0; col < 7; col++) {
				const cellIndex = row * 7 + col;
				const cellValue = board[cellIndex];
				cells.push(
					<td
						className="border border-slate-300 m-1"
						key={col}
						onClick={() => handleDropDisc(col)}
						onKeyUp={(e) => {
							if (e.key === "Enter") {
								handleDropDisc(col);
							}
						}}
						onKeyDown={(e) => {
							if (e.key === "Enter") {
								handleDropDisc(col);
							}
						}}
						style={{
							width: 50,
							height: 50,
							backgroundColor:
								cellValue === 1 ? "red" : cellValue === 2 ? "yellow" : "white",
							borderRadius: "50%",
						}}
					/>,
				);
			}
			rows.push(<tr key={row}>{cells}</tr>);
		}
		return rows;
	};

	return (
		<div className="flex flex-col items-center">
			<div>
				<h1>Connect4</h1>
				<table>
					<tbody className="border border-separate">{renderBoard()}</tbody>
				</table>
				{gameOver && (
					<div>
						<h2>
							{winner === 0
								? "Draw"
								: winner === 1
									? "Red wins!"
									: "Yellow wins!"}
						</h2>
						<Button onClick={resetGame}>もう一度</Button>
					</div>
				)}
			</div>
		</div>
	);
};

export default Connect4;
