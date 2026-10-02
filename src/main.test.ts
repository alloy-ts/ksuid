import { expect, test } from "vite-plus/test";
import { Base36, Base62, CrockfordBase32, Ksuid, KsuidMs } from "./main.ts";

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

test("Base62 utility class with seed-based shuffling and encoding", () => {
  const defaultAlph = Base62.defaultAlphabet();
  expect(defaultAlph).toBe("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
  expect(defaultAlph.length).toBe(62);

  const shuffled1 = Base62.shuffleAlphabet("seed123");
  const shuffled2 = Base62.shuffleAlphabet("seed123");
  expect(shuffled1).toBe(shuffled2);
  expect(shuffled1.length).toBe(62);
  expect(shuffled1).not.toBe(defaultAlph);

  const b62 = new Base62(shuffled1);
  const buf = Buffer.from("hello world base62");
  const encoded = b62.encode(buf);
  const decoded = b62.decode(encoded, buf.length);
  expect(decoded.toString()).toBe("hello world base62");

  const numEnc = b62.encodeNumber(9876543210);
  expect(b62.decodeNumber(numEnc)).toBe(9876543210);
});

test("Base36 class with seed-based shuffling and encoding", () => {
  const defaultAlph = Base36.defaultAlphabet();
  expect(defaultAlph).toBe("0123456789abcdefghijklmnopqrstuvwxyz");

  const shuffled1 = Base36.shuffleAlphabet("seed36");
  const shuffled2 = Base36.shuffleAlphabet("seed36");
  expect(shuffled1).toBe(shuffled2);
  expect(shuffled1.length).toBe(36);
  expect(shuffled1).not.toBe(defaultAlph);

  const b36 = new Base36(shuffled1);
  const buf = Buffer.from("hello world base36");
  const encoded = b36.encode(buf);
  const decoded = b36.decode(encoded, buf.length);
  expect(decoded.toString()).toBe("hello world base36");

  const numEnc = b36.encodeNumber(123456789);
  expect(b36.decodeNumber(numEnc)).toBe(123456789);

  const bytes128 = Buffer.alloc(16, 0xab);
  const enc128 = b36.encode128(bytes128);
  const dec128 = b36.decode128(enc128);
  expect(Array.from(dec128)).toEqual(Array.from(bytes128));
});

test("Seed-based deterministic alphabet shuffling works for all encoding algorithms", () => {
  const seed = "global-seed-42";
  const b62Alph = Base62.shuffleAlphabet(seed);
  const b36Alph = Base36.shuffleAlphabet(seed);
  const b32Alph = CrockfordBase32.shuffleAlphabet(seed);

  expect(b62Alph.length).toBe(62);
  expect(b36Alph.length).toBe(36);
  expect(b32Alph.length).toBe(32);

  const ksuid = Ksuid.now();

  const str62 = ksuid.toBase62(b62Alph);
  const kFrom62 = Ksuid.fromBase62(str62, b62Alph);
  expect(kFrom62.bytes().equals(ksuid.bytes())).toBe(true);

  const str36 = ksuid.toBase36(b36Alph);
  const kFrom36 = Ksuid.fromBase36(str36, b36Alph);
  expect(kFrom36.bytes().equals(ksuid.bytes())).toBe(true);

  const str32 = ksuid.toBase32(b32Alph);
  const kFrom32 = Ksuid.fromBase32(str32, b32Alph);
  expect(kFrom32.bytes().equals(ksuid.bytes())).toBe(true);
});

test("Ksuid and KsuidMs support custom shuffled alphabets via options and toString", () => {
  const seed = "ms-seed";
  const b36Alph = Base36.shuffleAlphabet(seed);

  const ksuid = new Ksuid({ enc: "base36", alphabet: b36Alph });
  const str = ksuid.toString();
  expect(ksuid.toBase36()).toBe(str);

  const k2 = new Ksuid(str, { enc: "base36", alphabet: b36Alph });
  expect(k2.equals(ksuid)).toBe(true);

  const ms = KsuidMs.now();
  const msStr36 = ms.toBase36(b36Alph);
  const ms2 = KsuidMs.fromBase36(msStr36, b36Alph);
  expect(ms2.equals(ms)).toBe(true);
});

test("Alphabet length validation error handling", () => {
  expect(() => new Base62("short")).toThrow();
  expect(() => new Base36("invalid")).toThrow();
  expect(() => new CrockfordBase32("invalid")).toThrow();
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
