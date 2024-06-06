import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";

export const metadata: Metadata = {
	title: "Connect 4",
	description: "A simple Connect 4 game",
};

const HonyaJi = localFont({
	src: "HonyaJi-Re.ttf",
	display: "swap",
});

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="ja" className={HonyaJi.className}>
			<body>{children}</body>
		</html>
	);
}
