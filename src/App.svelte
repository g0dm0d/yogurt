<script lang="ts">
  import { SvelteToast } from '@zerodevx/svelte-toast'
  import Left from './Left.svelte'

  const options = {}
</script>

<main class="container">
  <SvelteToast {options} />
  <div class="column left">
    <Left/>
  </div>
  <div class="column middle">
    <p>asddasasddassda</p>
  </div>
  <div class="column right">
    <p>asddasasddassda</p>
  </div>
</main>