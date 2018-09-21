const webpackConfig = require('@progre/webpack-config');

module.exports = (_, argv) => {
  const isProduction = argv.mode === 'production';
  return [
    webpackConfig.client(
      isProduction,
      'public/js/',
      ['randomizer.ts'],
      null,
    ),
    webpackConfig.electronRenderer(
      isProduction,
      'public/js/',
      ['index.tsx'],
      '.',
    ),
    webpackConfig.electronMain(
      isProduction,
      '.',
      ['index.ts'],
    ),
  ];
};
