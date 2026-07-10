import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// wtf is TS doing son... why do i need to recreate Object.keys()...
export function keys<A extends {}>(obj: A): Array<keyof A> {
  let keys: Array<keyof A> = []
  for (const key in obj) {
    keys.push(key)
  }
  return keys
}

const UNITS = {
  year: 24 * 60 * 60 * 1000 * 365,
  month: 24 * 60 * 60 * 1000 * 365/12,
  day: 24 * 60 * 60 * 1000,
  hour: 60 * 60 * 1000,
  minute: 60 * 1000,
  second: 1000,
} as const
export function getRelativeTime(then: Date) {
  const elapsed = then.getTime() - Date.now()
  const rtf = new Intl.RelativeTimeFormat("en", {
    numeric: "auto",
  })

  for (const unit of keys(UNITS)) {
    if (Math.abs(elapsed) > UNITS[unit] || unit == "second")
      return rtf.format(Math.round(elapsed / UNITS[unit]), unit)
  }
}

export const isFormDataString = (fd: FormDataEntryValue | null): fd is string => {
    return typeof fd === "string"
}