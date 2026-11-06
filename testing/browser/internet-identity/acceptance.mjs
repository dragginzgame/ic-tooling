import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { build } from 'esbuild';
import { chromium } from 'playwright-core';
import { withFixtureSession } from '../session.mjs';
import { pins } from './inputs.mjs';

const [driver, serverBinary, didc, inputs, evidence, executable] = process.argv.slice(2).map(value => resolve(value));
assert.equal(process.argv.length, 8, 'usage: node acceptance.mjs DRIVER SERVER DIDC INPUTS EVIDENCE CHROMIUM');
assert.equal(process.versions.node, '24.21.0', 'use the selected Node runtime');
const manifest = JSON.parse(await readFile(new URL('./package.json', import.meta.url), 'utf8'));
const versions = {};
for (const [name, version] of Object.entries(manifest.devDependencies)) {
  versions[name] = JSON.parse(await readFile(new URL(`./node_modules/${name}/package.json`, import.meta.url), 'utf8')).version;
  assert.equal(versions[name], version, `use the locked ${name} version`);
}
const tools = {};
for (const [name, path] of Object.entries({ driver, server: serverBinary, didc, chromium: executable })) {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  tools[name] = { path, sha256: hash.digest('hex') };
}
await mkdir(evidence); // Preserve every attempt; never replace an earlier proof.
const connection = await withFixtureSession({ command: driver,
  args: ['--server', serverBinary, '--didc', didc, '--inputs', inputs, '--output', join(evidence, 'fixture')],
  readyPath: join(evidence, 'fixture/connection.json'), logPath: join(evidence, 'fixture.log'),
  startupMs: 180_000, stopMs: 10_000, pollMs: 50,
}, async input => {
  assert.equal(input.schema, 1); assert.equal(input.dummyAuth, false); assert.equal(input.fetchRootKey, true);
  assert.equal(input.revision, pins.revision);
  const serverUrl = new URL(input.serverUrl);
  assert.equal(serverUrl.protocol, 'http:');
  assert.ok(['127.0.0.1', 'localhost'].includes(serverUrl.hostname));
  assert.ok(Number.isSafeInteger(input.instanceId) && input.instanceId >= 0);
  const instancesResponse = await fetch(new URL('instances', serverUrl), { signal: AbortSignal.timeout(5_000) });
  assert.equal(instancesResponse.status, 200);
  const instances = await instancesResponse.json();
  assert.ok(['Available', 'Busy'].includes(instances[input.instanceId]), 'attachment selects a live instance');
  for (const [field, name] of [['backendModuleHash', 'backend.wasm.gz'], ['frontendModuleHash', 'frontend.wasm.gz']]) {
    assert.equal(Buffer.from(input[field]).toString('hex'), pins.assets.find(asset => asset.name === name).sha256);
  }
  const gateway = new URL(input.gateway);
  assert.equal(gateway.protocol, 'http:'); assert.equal(gateway.hostname, 'localhost');
  assert.equal(input.authorizeUrl, `http://${input.frontendCanisterId}.localhost:${gateway.port}/authorize`);
  const bundle = await build({ entryPoints: [fileURLToPath(new URL('./browser.mjs', import.meta.url))], bundle: true,
    platform: 'browser', format: 'iife', globalName: 'iiFixture', write: false });
  const allowed = new Set([input.backendCanisterId, input.callerCanisterId]);
  const statuses = [];
  const server = createServer(async (request, response) => {
    try {
      const path = new URL(request.url, 'http://localhost').pathname;
      if (path === '/' && request.method === 'GET') {
        response.setHeader('content-type', 'text/html');
        response.end('<button id="sign-in">Sign in</button><script src="/bundle.js"></script>'); return;
      }
      if (path === '/bundle.js' && request.method === 'GET') {
        response.setHeader('content-type', 'text/javascript'); response.end(bundle.outputFiles[0].contents); return;
      }
      const match = /^\/api\/v[234]\/canister\/([^/]+)\/(call|read_state|query)$/.exec(path);
      if (request.method !== 'POST' || !match || !allowed.has(match[1])) {
        response.writeHead(404, { 'access-control-allow-origin': '*' }); response.end(); return;
      }
      const chunks = []; let size = 0;
      for await (const chunk of request) {
        size += chunk.length; assert.ok(size <= 1_048_576); chunks.push(chunk);
      }
      const reply = await fetch(new URL(path, gateway), { method: 'POST', body: Buffer.concat(chunks),
        headers: { 'content-type': 'application/cbor' }, signal: AbortSignal.timeout(20_000) });
      const bytes = Buffer.from(await reply.arrayBuffer());
      statuses.push({ path, status: reply.status });
      response.writeHead(reply.status, { 'content-type': reply.headers.get('content-type') ?? 'application/cbor' });
      response.end(bytes);
    } catch (error) { console.error(error); response.writeHead(502); response.end(); }
  });
  server.requestTimeout = 25_000;
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  let browser;
  try {
    browser = await chromium.launch({ executablePath: executable, headless: true });
    const context = await browser.newContext();
    await context.tracing.start({ screenshots: true, snapshots: true, sources: false });
    // All browser traffic must stay local, including failures. No production II,
    // OIDC provider, analytics or physical/personal authenticator is contacted.
    await context.route('**/*', route => {
      const url = new URL(route.request().url());
      return url.protocol === 'http:' && (url.hostname === '127.0.0.1' || url.hostname === 'localhost' || url.hostname.endsWith('.localhost'))
        ? route.continue() : route.abort('blockedbyclient');
    });
    const page = await context.newPage();
    context.on('page', page => {
      page.on('pageerror', error => console.error('browser error:', error.message));
      page.on('console', message => { if (message.type() === 'error') console.error('browser console:', message.text()); });
      page.on('dialog', async dialog => { console.error('unexpected dialog:', dialog.message()); await dialog.dismiss(); });
    });
    page.setDefaultTimeout(30_000);
    await page.goto(`http://127.0.0.1:${server.address().port}`);
    await page.evaluate(input => {
      iiFixture.open(input);
      document.querySelector('#sign-in').onclick = () => {
        iiFixture.signIn().then(result => { window.outcome = { result }; }, error => { window.outcome = { error: String(error) }; });
      };
    }, input);
    const popupPromise = page.waitForEvent('popup', { timeout: 30_000 });
    await page.locator('#sign-in').click();
    const popup = await popupPromise;
    popup.setDefaultTimeout(30_000);
    const cdp = await context.newCDPSession(popup);
    await cdp.send('WebAuthn.enable');
    const { authenticatorId } = await cdp.send('WebAuthn.addVirtualAuthenticator', { options: {
      protocol: 'ctap2', transport: 'usb', hasResidentKey: true, hasUserVerification: true,
      isUserVerified: true, automaticPresenceSimulation: true,
    } });
    await popup.getByRole('button', { name: 'Create', exact: true }).click();
    await popup.getByRole('button', { name: 'Create with passkey' }).click();
    await popup.getByLabel('Identity name').fill('Disposable Tooling acceptance');
    await popup.getByRole('button', { name: 'Create identity' }).click();
    // Retain only public WebAuthn evidence, never virtual private-key material.
    const { credentials } = await cdp.send('WebAuthn.getCredentials', { authenticatorId });
    assert.equal(credentials.length, 1, 'II registered a real virtual passkey');
    assert.ok(credentials[0].signCount > 0, 'II used the authenticator to sign');
    const webauthn = credentials.map(({ credentialId, rpId, signCount }) => ({ credentialId, rpId, signCount }));
    await popup.getByRole('button', { name: 'Continue', exact: true }).click();
    await page.waitForFunction(() => window.outcome, undefined, { timeout: 30_000 });
    const outcome = await page.evaluate(() => window.outcome);
    assert.equal(outcome.error, undefined, outcome.error);
    const { result } = outcome;
    assert.equal(result.principal, result.observed);
    assert.ok(statuses.some(item => item.status === 400 && item.path.includes(input.callerCanisterId)), 'replica rejected the forged signature');
    await context.tracing.stop({ path: join(evidence, 'browser.zip') });
    await writeFile(join(evidence, 'proof.json'), JSON.stringify({ ...result, webauthn, statuses,
      attachment: { serverUrl: input.serverUrl, instanceId: input.instanceId, state: instances[input.instanceId] },
      ii: pins, versions, tools, node: process.versions.node, chromium: browser.version(),
      assurance: 'Chromium virtual WebAuthn integration; no physical authenticator assurance',
    }, null, 2), { flag: 'wx' });
    console.log(`II real signIn/getIdentity and signed ingress passed: ${result.principal} (Chromium ${browser.version()})`);
    return input;
  } catch (error) {
    for (const page of browser?.contexts()[0]?.pages() ?? []) {
      console.error('Failed page:', page.url(), await page.locator('body').innerText().catch(() => 'unavailable'));
    }
    await browser?.contexts()[0]?.tracing.stop({ path: join(evidence, 'browser-failed.zip') });
    throw error;
  } finally {
    await browser?.close();
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
});
const stoppedGateway = new URL(connection.gateway); stoppedGateway.hostname = '127.0.0.1';
await assert.rejects(fetch(stoppedGateway, { signal: AbortSignal.timeout(2_000) }),
  error => error.cause?.code === 'ECONNREFUSED', 'fixture teardown closes the gateway');
await assert.rejects(fetch(new URL('instances', connection.serverUrl), { signal: AbortSignal.timeout(2_000) }),
  error => error.cause?.code === 'ECONNREFUSED', 'fixture teardown stops the management server');
await writeFile(join(evidence, 'cleanup.json'), JSON.stringify({ driverExit: 0, gatewayRefused: true, serverRefused: true }), { flag: 'wx' });
