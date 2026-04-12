import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'

import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'
import { theme } from '../theme/dark'

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi'
  },
  theme: {
    themes: {
      dark: {
        colors: theme.colors,
      }
    },
  },
  defaults: {
    VAppBar: {
      elevateOnScroll: true,
      flat: false,
    },
    VBtn: {
      rounded: 'lg',
    },
    VTextField: {
      variant: 'outlined',
      density: 'comfortable',
    },
    VCard: {
      elevation: 3,
      rounded: 'xl',
    },
    VTabs: {
        height: 55,
    },
    VDateInput: {
        density: 'compact',
        variant: 'solo-filled',
        rounded: 'lg',
    },
  },
})

export default vuetify
