export const AuthType = {
  None: 'NONE',
  Basic: 'BASIC',
  Bearer: 'BEARER',
} as const;

export type AuthType = typeof AuthType[keyof typeof AuthType];
