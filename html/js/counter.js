import { ref, onMounted } from 'vue'

export default {
    setup() {
        const count = ref(0)
        return { count }
    },

    mounted() {
      console.log(`The initial count is ${this.count}.`)
    },

    template: `
        <button @click="count++">{{ count }}</button>
    `,
}
