import { expect, test } from "vite-plus/test";
import { CrockfordBase32, Ksuid, KsuidMs } from "./main.ts";

test("Ksuid creates valid base62 string", () => {
  const ksuid = Ksuid.now();
  expect(typeof ksuid.toBase62()).toBe("string");
  expect(ksuid.toBase62().length).toBe(27);
  expect(Ksuid.isValid(ksuid.toBase62())).toBe(true);
  expect(ksuid.timestampSize).toBe("32bit");
});

test("Ksuid supports timestampSize option (32bit and 48bit)", () => {
  const k32 = new Ksuid({ timestampSize: "32bit" });
  expect(k32.timestampSize).toBe("32bit");
  expect(k32.payloadBytes).toBe(16);

  const k48 = new Ksuid({ timestampSize: "48bit" });
  expect(k48.timestampSize).toBe("48bit");
  expect(k48.payloadBytes).toBe(15);

  const k48WithTs = new Ksuid({ timestampSize: "48bit", timestamp: 1700000000000 });
  expect(k48WithTs.timestampSize).toBe("48bit");
});

test("Ksuid supports Crockford Base32 encoding and options", () => {
  const ksuid = Ksuid.now();
  const b32 = ksuid.toCrockfordBase32();
  expect(b32.length).toBe(32);
  expect(ksuid.toString("base32")).toBe(b32);
  expect(ksuid.toString({ enc: "base32" })).toBe(b32);

  const ksuid2 = Ksuid.fromCrockfordBase32(b32);
  expect(ksuid2.toBase62()).toBe(ksuid.toBase62());
});

test("CrockfordBase32 utility with custom and shuffled alphabet", () => {
  const defaultAlph = CrockfordBase32.defaultAlphabet();
  expect(defaultAlph).toBe("0123456789ABCDEFGHJKMNPQRSTVWXYZ");

  const shuffled = CrockfordBase32.shuffleAlphabet("test-seed");
  expect(shuffled.length).toBe(32);

  const encoder = new CrockfordBase32(shuffled);
  const numStr = encoder.encodeNumber(99999);
  expect(encoder.decodeNumber(numStr)).toBe(99999);
});

test("Ksuid base62 parsing and formatting", () => {
  const base62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
  const ksuid = Ksuid.fromBase62(base62);
  expect(ksuid.toBase62()).toBe(base62);
  expect(ksuid.toString()).toBe(base62);
});

test("Ksuid payload and bytes length", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.bytes().length).toBe(20);
  expect(ksuid.payload().length).toBe(16);
});

test("Ksuid timestamp and comparison", () => {
  const k1 = Ksuid.fromSeconds(1555555555);
  const k2 = Ksuid.fromSeconds(1777777777);
  expect(k1.timestampSeconds()).toBe(1555555555);
  expect(k2.timestampSeconds()).toBe(1777777777);
  expect(k1.compare(k2)).toBeLessThan(0);
  expect(k2.compare(k1)).toBeGreaterThan(0);
  expect(k1.equals(k1)).toBe(true);
});

test("KsuidMs creates valid instance", () => {
  const ksuidMs = KsuidMs.now();
  expect(typeof ksuidMs.toBase62()).toBe("string");
  expect(ksuidMs.bytes().length).toBe(20);
  expect(ksuidMs.payload().length).toBe(15);
});
