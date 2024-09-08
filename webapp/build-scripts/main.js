const esbuild = require('esbuild');


esbuild.build({
  entryPoints: ['../main/main.ts'],
  bundle: true,
  outfile: '../dist/main.js',
  target: ['chrome68'],
  sourcemap : true,
  minify : true
});



