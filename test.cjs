const assert = require("node:assert/strict");
const test = require("node:test");

const { Ksuid, KsuidMs } = require("./dist/index.js");

void test("Ksuid - creation and base62", () => {
  const ksuid = Ksuid.now();
  assert.equal(typeof ksuid.toBase62(), "string");
  assert.equal(ksuid.toBase62().length, 27);
  assert.equal(ksuid.toString(), ksuid.toBase62());
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
