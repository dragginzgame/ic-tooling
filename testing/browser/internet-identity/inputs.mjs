import { createHash } from 'node:crypto';
import { mkdir, open, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { constants } from 'node:fs';

export const pins = JSON.parse(await readFile(new URL('./inputs.json', import.meta.url), 'utf8'));
const limit = 32 * 1024 * 1024;

export function admit(asset, bytes) {
  if (bytes.length > limit || createHash('sha256').update(bytes).digest('hex') !== asset.sha256) {
    throw new Error(`II input checksum/size mismatch: ${asset.name}`);
  }
  return bytes;
}

export async function readPrepared(asset, path) {
  const file = await open(path, constants.O_RDONLY | constants.O_NOFOLLOW | constants.O_NONBLOCK);
  try {
    const stat = await file.stat();
    if (!stat.isFile() || stat.size > limit) throw new Error(`II input must be a bounded regular file: ${asset.name}`);
    const { buffer, bytesRead } = await file.read({ buffer: Buffer.alloc(stat.size + 1), position: 0 });
    return admit(asset, buffer.subarray(0, bytesRead));
  } finally { await file.close(); }
}

// Explicit setup only; the Rust fixture rechecks every input offline before use.
async function setup(directory) {
  await mkdir(directory, { recursive: true });
  for (const asset of pins.assets) {
    const path = join(directory, asset.name);
    try { await readPrepared(asset, path); continue; }
    catch (error) { if (error.code !== 'ENOENT') throw error; }
    const response = await fetch(asset.url, { signal: AbortSignal.timeout(60_000) });
    if (!response.ok) throw new Error(`${asset.name}: HTTP ${response.status}`);
    const chunks = []; let size = 0;
    for await (const chunk of response.body) {
      size += chunk.length;
      if (size > limit) throw new Error(`${asset.name}: download exceeds ${limit} bytes`);
      chunks.push(chunk);
    }
    const bytes = admit(asset, Buffer.concat(chunks));
    const file = await open(path, 'wx', 0o600);
    try { await file.writeFile(bytes); } finally { await file.close(); }
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.length !== 4 || process.argv[2] !== 'setup') throw new Error('usage: node inputs.mjs setup DIRECTORY');
  await setup(process.argv[3]);
}
