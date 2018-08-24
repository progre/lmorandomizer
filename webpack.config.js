const webpackConfig = require('@progre/webpack-config');

module.exports = (_, argv) => {
  const isProduction = argv.mode === 'production';
  return [
    webpackConfig.electronRenderer(
      isProduction,
      'public/js/',
      ['index.tsx'],
      'public/',
    ),
    webpackConfig.electronMain(
      isProduction,
      '.',
      ['index.ts'],
    ),
  ];
};
