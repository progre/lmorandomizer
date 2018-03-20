const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const uglifySaveLicense = require('uglify-save-license');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const electronVersion = require('./package.json').devDependencies.electron.slice(1);

const isProduction = process.env.NODE_ENV === 'production';

const common = {
  devtool: isProduction ? false : 'inline-source-map',
  node: { __filename: true, __dirname: false },
  resolve: { extensions: ['.ts', '.tsx', '.js'] },
  watchOptions: { ignored: /node_modules|dist/ },
};

const tsLoader = {
  rules: [{
    test: /\.tsx?$/,
    use: [
      {
        loader: 'ts-loader',
        options: { compilerOptions: { sourceMap: !isProduction } }
      }
    ]
  }]
};

const clientSide = {
  entry: {
    index: './src/public/js/index.ts'
  },
  externals: /^electron$/,
  module: tsLoader,
  output: { filename: 'dist/public/js/[name].js', libraryTarget: 'commonjs2' },
  plugins: [
    new CopyWebpackPlugin(
      [{ from: 'src/public/', to: 'dist/public/' }],
      { ignore: ['test/', '*.ts', '*.tsx'] },
    ),
    ...(
      !isProduction ? [] : [
        new UglifyJsPlugin({
          uglifyOptions: { output: { comments: uglifySaveLicense } },
        }),
      ]
    )
  ],
  target: 'electron-renderer',
};

const serverSide = {
  entry: {
    index: './src/index.ts'
  },
  externals: /^(?!\.)/,
  module: tsLoader,
  output: { filename: 'dist/[name].js', libraryTarget: 'commonjs2' },
  target: 'electron-main',
};

module.exports = [
  { ...common, ...clientSide },
  { ...common, ...serverSide },
];
