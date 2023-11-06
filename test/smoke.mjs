import test from 'node:test';
import assert from 'node:assert';

import './smoke.bc.js';

const smoke = globalThis.smoke;

test('smoke.hello should contain "Hello, world!"', () => {
  assert.strictEqual(smoke.hello, 'Hello, world!');
});

test('smoke.add() should add two numbers', () => {
  assert.strictEqual(smoke.add(1, 1), 2);
});
