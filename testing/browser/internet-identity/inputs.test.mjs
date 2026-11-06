import assert from 'node:assert/strict';
import { test } from 'node:test';
import { createHash } from 'node:crypto';
import { open, rm, symlink, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { admit, readPrepared } from './inputs.mjs';
import { scratchDirectory } from '../tests/scratch.mjs';

const bytes = Buffer.from('immutable fixture bytes');
const asset = { name: 'fixture', sha256: createHash('sha256').update(bytes).digest('hex') };

test('immutable II inputs refuse changed, truncated and oversized bytes', () => {
  assert.equal(admit(asset, bytes), bytes);
  assert.throws(() => admit(asset, Buffer.from('changed fixture bytes')));
  assert.throws(() => admit(asset, bytes.subarray(1)));
  assert.throws(() => admit(asset, Buffer.alloc(32 * 1024 * 1024 + 1)));
});

test('prepared inputs refuse missing files, symlinks and oversized regular files', async t => {
  const directory = await scratchDirectory('ii-inputs-');
  t.after(() => rm(directory, { recursive: true }));
  const path = join(directory, 'fixture');
  await assert.rejects(readPrepared(asset, path), { code: 'ENOENT' });
  await writeFile(path, bytes);
  assert.deepEqual(await readPrepared(asset, path), bytes);
  await symlink(path, join(directory, 'link'));
  await assert.rejects(readPrepared(asset, join(directory, 'link')), { code: 'ELOOP' });
  const file = await open(path, 'r+');
  try { await file.truncate(32 * 1024 * 1024 + 1); } finally { await file.close(); }
  await assert.rejects(readPrepared(asset, path), /bounded regular file/);
});
