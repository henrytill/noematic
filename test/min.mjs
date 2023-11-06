import assert from 'node:assert';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';

import { JSDOM, VirtualConsole } from 'jsdom';

const MIN_HTML = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1.0">
  <title>Brr minimal example</title>
  <script defer src="./min.bc.js"></script>
</head>
<body>
  <noscript>Sorry, you need to enable JavaScript to see this page.</noscript>
</body>
</html>
`;

const dirname = path.dirname(url.fileURLToPath(import.meta.url));

console.log(`dirname: ${dirname}`);

const virtualConsole = new VirtualConsole().sendTo(console);

test('The Brr minimal example works', async () => {
  const dom = new JSDOM(MIN_HTML, {
    runScripts: 'dangerously',
    resources: 'usable',
    url: `file://${dirname}/index.html`,
    virtualConsole,
  });

  const { window } = dom;

  await new Promise((resolve, _) => {
    window.addEventListener('load', () => resolve());
  });

  const { document } = window;
  assert.strictEqual(document.title, 'Brr minimal example');
  assert.strictEqual(document.body.textContent, 'Hello, world!');
});
