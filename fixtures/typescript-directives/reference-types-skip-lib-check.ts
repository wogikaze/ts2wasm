// @filename: /node_modules/foo/index.d.ts
/// <reference types="cookie-session"/>
export const foo = 1;

// @filename: /tsconfig.json
{
    "compilerOptions": {
        "strict": true,
        "skipLibCheck": true
    }
}
