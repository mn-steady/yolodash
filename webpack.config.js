const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/shade-import.js',
  output: {
    filename: 'shade.bundle.js',
    path: path.resolve(__dirname, 'static/lib/shade'),
  },
  mode: 'production',
  resolve: {
    extensions: ['.js'],
    modules: [path.resolve(__dirname, 'node_modules')],
  },
  plugins: [
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'],
    }),
  ],
};
