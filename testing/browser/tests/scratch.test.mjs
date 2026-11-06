import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { access, mkdir, realpath, rm, symlink } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { scratchDirectory } from './scratch.mjs';

test('browser source fixtures default to target scratch and preserve explicit physical parents', async t => {
  const fixture = await scratchDirectory('browser-scratch-test-');
  const children = [];
  t.after(async () => {
    for (const child of children) await rm(child, { recursive: true });
    await rm(fixture, { recursive: true });
  });
  await mkdir(join(fixture, 'override'));
  await symlink(join(fixture, 'override'), join(fixture, 'alias'));
  const target = fileURLToPath(new URL('../../../target/tmp/', import.meta.url));
  await mkdir(target, { recursive: true });
  for (const [selection, expected] of [[undefined, target], ['alias', join(fixture, 'override')], [join(fixture, 'invalid\nparent'), null]]) {
    const environment = { ...process.env };
    delete environment.TMPDIR;
    if (selection) environment.TMPDIR = selection;
    const run = () => execFileSync(process.execPath, ['--input-type=module', '-e',
      `import { scratchDirectory } from ${JSON.stringify(new URL('./scratch.mjs', import.meta.url).href)}; process.stdout.write(await scratchDirectory('browser-child-'));`],
    { cwd: fixture, env: environment, timeout: 3_000, encoding: 'utf8', stdio: 'pipe' });
    if (expected === null) {
      assert.throws(run, { status: 1 });
      await assert.rejects(access(selection), { code: 'ENOENT' });
      continue;
    }
    const directory = run();
    children.push(directory);
    assert.equal(dirname(directory), await realpath(expected));
  }
});
