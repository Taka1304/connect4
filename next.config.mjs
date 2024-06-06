/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack(config, { isServer, dev }) {
		config.experiments = {
			...config.experiments,
			asyncWebAssembly: true,
			layers: true,
		};
		config.output.webassemblyModuleFilename =
			isServer && !dev
				? "../static/pkg/[modulehash].wasm"
				: "static/pkg/[modulehash].wasm";

		return config;
	},
};

export default nextConfig;
