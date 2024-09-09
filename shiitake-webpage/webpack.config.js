const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlInlineScriptPlugin = require('html-inline-script-webpack-plugin');

// Base config that applies to either development or production mode.
const config = {
    entry: './src/index.ts',
    output: {
        // Compile the source files into a bundle.
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
        clean: true,

    },
    // Enable webpack-dev-server to get hot refresh of the app.
    devServer: {
        static: './build',
        hot: true,
        watchFiles: ['src/**/*.html'],
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                // Load CSS files. They can be imported into JS files.
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js', '.css', '.html'],
    },
    plugins: [
        // Generate the HTML index page based on our template.
        // This will output the same index page with the bundle we
        // created above added in a script tag.
        new HtmlWebpackPlugin({
            template: 'src/index.html'
        })
    ],
};

module.exports = (env, argv) => {
    if (argv.mode === 'development') {
        // Set the output path to the `build` directory
        // so we don't clobber production builds.
        config.output.path = path.resolve(__dirname, 'build');
    } else if (argv.mode === 'production') {
        // Inline the bundle into the HTML page.
        config.plugins.push(new HtmlInlineScriptPlugin());
    }
    return config;
};
