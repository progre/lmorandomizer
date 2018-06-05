const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const uglifySaveLicense = require('uglify-save-license');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');

module.exports = (_, argv) => {
  const isProduction = argv.mode === 'production';

  const common = {
    devtool: isProduction ? false : 'inline-source-map',
    node: { __dirname: true, __filename: true },
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
    module: tsLoader,
    output: { filename: 'public/js/[name].js' },
    plugins: [
      new CopyWebpackPlugin(
        [{ from: 'src/public/', to: 'public/' }],
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
    target: 'web',
  };

  const serverSide = {
    entry: {
      index: './src/index.ts'
    },
    externals: /^(?!\.)/,
    module: tsLoader,
    output: { filename: '[name].js', libraryTarget: 'commonjs2' },
    target: 'node',
  };

  return [
    { ...common, ...clientSide },
    { ...common, ...serverSide },
  ];
};
