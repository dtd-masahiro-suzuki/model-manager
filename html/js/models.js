import { createApp, ref } from 'vue'
import Counter from './counter.js'

createApp({
    components: { Counter },
    setup() {
      const message = ref('Hello vue!')
      return {
        message
      }
    }
}).mount('#app')
