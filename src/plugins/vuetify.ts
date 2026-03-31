import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { theme } from '../theme/dark'

const vuetify = createVuetify({
  components,
  directives,
  theme: {
    themes: {
      light: {
        colors: theme.colors,
      },
      dark: {
        colors: theme.colors,
      }
    },
  },
  defaults: {
    VAppBar: {
      color: 'primary',
      elevateOnScroll: true,
      flat: false,
    },
    VBtn: {
      color: 'primary',
      rounded: 'lg',
      elevation: 2,
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
  },
})

export default vuetify
