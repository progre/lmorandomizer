const webpackConfig = require('@progre/webpack-config');

module.exports = (_, argv) => {
  const isProduction = argv.mode === 'production';
  return [
    webpackConfig.client(
      isProduction,
      'public/js/',
      ['index.ts'],
      true,
    ),
    webpackConfig.server(
      isProduction,
      '.',
      ['index.ts'],
    ),
  ];
};
