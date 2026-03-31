import type { Config } from 'tailwindcss'
import { theme } from './src/theme/dark'

export default {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: theme.colors,
      fontFamily: {
        sans: theme.typography.fontFamily,
      },
      fontSize: theme.typography.fontSize,
      fontWeight: theme.typography.fontWeight,
      spacing: theme.spacing,
    },
  },
  plugins: [],
} satisfies Config
