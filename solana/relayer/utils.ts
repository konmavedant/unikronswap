import { createHash } from "crypto";

export function hashSwapData(data: string): Buffer {
  return createHash("sha256").update(data).digest();
}
