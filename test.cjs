const assert = require("node:assert/strict");
const test = require("node:test");

const { Ksuid, KsuidMs, CrockfordBase32 } = require("./dist/index.js");

void test("Ksuid - creation and base62", () => {
  const ksuid = Ksuid.now();
  assert.equal(typeof ksuid.toBase62(), "string");
  assert.equal(ksuid.toBase62().length, 27);
  assert.equal(ksuid.toString(), ksuid.toBase62());
  assert.equal(ksuid.timestampSize, "32bit");
});

void test("Ksuid - timestampSize option (32bit vs 48bit)", () => {
  const ksuid32 = new Ksuid({ timestampSize: "32bit" });
  assert.equal(ksuid32.timestampSize, "32bit");
  assert.equal(ksuid32.payloadBytes, 16);

  const ksuid48 = new Ksuid({ timestampSize: "48bit" });
  assert.equal(ksuid48.timestampSize, "48bit");
  assert.equal(ksuid48.payloadBytes, 15);

  const ksuidCustomTs = new Ksuid({
    timestampSize: "48bit",
    timestamp: 1700000000000,
  });
  assert.equal(ksuidCustomTs.timestampSize, "48bit");
});

void test("Ksuid - Crockford Base32 encoding and options", () => {
  const ksuid = Ksuid.now();
  const b32 = ksuid.toCrockfordBase32();
  assert.equal(typeof b32, "string");
  assert.equal(b32.length, 32);

  assert.equal(ksuid.toString("base32"), b32);
  assert.equal(ksuid.toString({ enc: "base32" }), b32);

  const ksuidFromB32 = Ksuid.fromCrockfordBase32(b32);
  assert.equal(ksuidFromB32.toBase62(), ksuid.toBase62());

  const ksuidFromCtor = new Ksuid(b32);
  assert.equal(ksuidFromCtor.toBase62(), ksuid.toBase62());
});

void test("CrockfordBase32 class and shuffleAlphabet", () => {
  const defaultAlph = CrockfordBase32.defaultAlphabet();
  assert.equal(defaultAlph, "0123456789ABCDEFGHJKMNPQRSTVWXYZ");

  const shuffled = CrockfordBase32.shuffleAlphabet("my-seed");
  assert.equal(shuffled.length, 32);

  const cb32 = new CrockfordBase32(shuffled);
  const numStr = cb32.encodeNumber(12345);
  const numDec = cb32.decodeNumber(numStr);
  assert.equal(numDec, 12345);
});

void test("Ksuid - from base62 string", () => {
  const base62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
  const ksuid = Ksuid.fromBase62(base62);
  assert.equal(ksuid.toBase62(), base62);

  const ksuidFromStr = Ksuid.fromStr(base62);
  assert.equal(ksuidFromStr.toBase62(), base62);

  const ksuidCtor = new Ksuid(base62);
  assert.equal(ksuidCtor.toBase62(), base62);
});

void test("Ksuid - bytes and payload", () => {
  const ksuid = Ksuid.now();
  const bytes = ksuid.bytes();
  assert.equal(bytes.length, 20);

  const payload = ksuid.payload();
  assert.equal(payload.length, 16);

  const fromBytes = Ksuid.fromBytes(bytes);
  assert.equal(fromBytes.toBase62(), ksuid.toBase62());
});

void test("Ksuid - compare and equals", () => {
  const ksuid1 = Ksuid.fromSeconds(1555555555);
  const ksuid2 = Ksuid.fromSeconds(1777777777);

  assert.equal(ksuid1.compare(ksuid2), -1);
  assert.equal(ksuid2.compare(ksuid1), 1);
  assert.equal(ksuid1.compare(ksuid1), 0);

  assert.equal(ksuid1.equals(ksuid1), true);
  assert.equal(ksuid1.equals(ksuid2), false);
});

void test("KsuidMs - creation and methods", () => {
  const ms = KsuidMs.now();
  assert.equal(typeof ms.toBase62(), "string");
  assert.equal(ms.bytes().length, 20);
  assert.equal(ms.payload().length, 15);
});
