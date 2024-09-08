const esbuild = require('esbuild');

const proc = require('child_process');
const fs = require('fs');

process.chdir("../core");

proc.execSync('npm run build');

esbuild.build({
  entryPoints: ['../core/_tmp/index.js'],
  bundle: true,
  outfile: '../dist/core.js',
  target: ['chrome68'],
  platform : "browser",
  minifyIdentifiers: false,
  keepNames: true,
  define : {
    'global' : 'window'
  },
  globalName : "monCore"
});

fs.copyFileSync("./_tmp/index.d.ts", "../types/core.d.ts")



