import { expect, test } from "vite-plus/test";
import { Base36, CrockfordBase32, Ksuid, KsuidMs } from "./main.ts";

test("Ksuid creates valid base62 string", () => {
  const ksuid = Ksuid.now();
  expect(typeof ksuid.toBase62()).toBe("string");
  expect(ksuid.toBase62().length).toBe(27);
  expect(Ksuid.isValid(ksuid.toBase62())).toBe(true);
  expect(ksuid.timestampSize).toBe("32bit");
});

test("Ksuid supports timestampSize option (32bit, 48bit, 64bit)", () => {
  const k32 = new Ksuid({ timestampSize: "32bit" });
  expect(k32.timestampSize).toBe("32bit");
  expect(k32.payloadBytes).toBe(16);

  const k48 = new Ksuid({ timestampSize: "48bit" });
  expect(k48.timestampSize).toBe("48bit");
  expect(k48.payloadBytes).toBe(15);

  const k64 = new Ksuid({ timestampSize: "64bit" });
  expect(k64.timestampSize).toBe("64bit");
  expect(k64.payloadBytes).toBe(15);
});

test("Ksuid constructor options { enc: 'base32', alphabet, timestampSize }", () => {
  const customAlph = CrockfordBase32.shuffleAlphabet("test-seed-48");
  const ksuid = new Ksuid({
    enc: "base32",
    alphabet: customAlph,
    timestampSize: "48bit",
  });

  expect(ksuid.timestampSize).toBe("48bit");
  expect(ksuid.enc).toBe("base32");
  expect(ksuid.alphabet).toBe(customAlph);

  const str = ksuid.toString();
  expect(str.length).toBe(32);
  expect(ksuid.toCrockfordBase32(customAlph)).toBe(str);
});

test("Ksuid supports Base36 encoding", () => {
  const ksuid = new Ksuid({ enc: "base36" });
  expect(ksuid.enc).toBe("base36");
  const str = ksuid.toString();
  expect(typeof str).toBe("string");

  const k2 = Ksuid.fromBase36(str);
  expect(k2.toBase62()).toBe(ksuid.toBase62());
});

test("Base36 128-bit reference test vectors", () => {
  const zeroBytes = Buffer.alloc(16);
  expect(Base36.encode128(zeroBytes)).toBe("0000000000000000000000000");

  const vec1 = Buffer.from([
    0x01, 0x7f, 0xee, 0x7f, 0xef, 0x41, 0x7e, 0x2b, 0x34, 0x32, 0xac, 0x2e, 0xc5, 0x53, 0x68, 0x7c,
  ]);
  expect(Base36.encode128(vec1)).toBe("0372hg16csmsm50l8dikcvukc");

  const dec1 = Base36.decode128("0372hg16csmsm50l8dikcvukc");
  expect(Array.from(dec1)).toEqual(Array.from(vec1));

  const maxBytes = Buffer.alloc(16, 0xff);
  expect(Base36.encode128(maxBytes)).toBe("f5lxx1zz5pnorynqglhzmsp33");
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
