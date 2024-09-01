const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./core/src/index.ts'],
  bundle: true,
  outfile: './dist/core.js',
  target: ['chrome68'],
  sourcemap : true,
  minify : true
});

