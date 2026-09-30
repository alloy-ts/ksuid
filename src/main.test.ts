import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs } from "./main.ts";

test("Ksuid creates valid base62 string", () => {
  const ksuid = Ksuid.now();
  expect(typeof ksuid.toBase62()).toBe("string");
  expect(ksuid.toBase62().length).toBe(27);
  expect(Ksuid.isValid(ksuid.toBase62())).toBe(true);
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
