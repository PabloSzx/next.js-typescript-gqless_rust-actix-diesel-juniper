export const IS_BROWSER = typeof window !== "undefined";
export const IS_PRODUCTION = process.env.NODE_ENV === "production";
export const IS_NOT_PRODUCTION = !IS_PRODUCTION;
