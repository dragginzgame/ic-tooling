import { mkdir, mkdtemp, realpath } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

// Test-only source/child fixtures; callers retain cleanup and failure policy.
export async function scratchDirectory(prefix) {
  const parent = process.env.TMPDIR || fileURLToPath(new URL('../../../target/tmp/', import.meta.url));
  if (/[\r\n]/.test(parent)) throw new Error('scratch parent must not contain CR/LF');
  await mkdir(parent, { recursive: true });
  return mkdtemp(join(await realpath(resolve(parent)), prefix));
}
