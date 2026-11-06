import { AuthClient } from '@icp-sdk/auth/client';
import { HttpAgent, AgentError, CertificateVerificationErrorCode, HttpErrorCode } from '@icp-sdk/core/agent';
import { Principal } from '@icp-sdk/core/principal';

let client;
let input;
const require = (condition, message) => { if (!condition) throw new Error(message); };

export function open(connection) {
  input = connection;
  client = new AuthClient({
    identityProvider: { authorizeUrl: input.authorizeUrl, canisterId: input.backendCanisterId },
    agentOptions: { host: location.origin, rootKey: Uint8Array.from(input.rootKey), retryTimes: 0 },
  });
}

export async function signIn() {
  const signedIn = await client.signIn();
  const identity = await client.getIdentity();
  const principal = identity.getPrincipal();
  require(!principal.isAnonymous(), 'real II returns a nonanonymous identity');
  require(signedIn.getPrincipal().toText() === principal.toText(), 'signIn/getIdentity agree');
  const options = { host: location.origin, rootKey: Uint8Array.from(input.rootKey), identity, retryTimes: 0 };
  const call = agent => agent.update(Principal.fromText(input.callerCanisterId), { methodName: 'caller', arg: new Uint8Array() });
  const reply = await call(new HttpAgent(options));
  const observed = Principal.fromUint8Array(reply.reply).toText();
  require(observed === principal.toText(), 'certified ingress reports the II delegated caller');

  // Actual trust/admission controls. Neither mock responses nor verifier overrides.
  const wrongRoot = Uint8Array.from(input.rootKey); wrongRoot[wrongRoot.length - 1] ^= 1;
  let refusedRoot = false;
  try { await call(new HttpAgent({ ...options, rootKey: wrongRoot })); }
  catch (error) { refusedRoot = error instanceof AgentError && error.code instanceof CertificateVerificationErrorCode; }
  require(refusedRoot, 'the signed result must fail with a wrong local trust root');

  const forged = {
    getPrincipal: () => identity.getPrincipal(),
    transformRequest: async request => {
      const signed = await identity.transformRequest(request);
      const signature = Uint8Array.from(signed.body.sender_sig);
      require(signature.length > 0, 'real delegated ingress has a signature');
      signature[0] ^= 1;
      return { ...signed, body: { ...signed.body, sender_sig: signature } };
    },
  };
  let refusedSignature = false;
  let signatureError;
  try { await call(new HttpAgent({ ...options, identity: forged })); }
  catch (error) {
    signatureError = String(error);
    refusedSignature = error instanceof AgentError && error.code instanceof HttpErrorCode && error.code.status === 400;
  }
  require(refusedSignature, 'the replica must refuse the altered ingress signature');
  return { principal: principal.toText(), observed, refusedRoot, refusedSignature, signatureError };
}
