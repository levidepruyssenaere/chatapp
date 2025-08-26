<script lang="ts">
    export let iconPath: string;
    export let onClick: () => void;

  // Props go here
  // export let propName = defaultValue;
  
  // Local variables
  // let localVar = initialValue;
  
  // Functions/methods
  // function handleSomething() {
  //   // code here
  // }
  
  // Lifecycle hooks if needed
  // import { onMount } from 'svelte';
  // onMount(() => {
  //   // code that runs when component is mounted
  // });
</script>

<!-- HTML markup goes here -->
 <button onclick={onClick}>
    <img src={iconPath} alt="">
 </button>

<style>
  /* CSS styles for this component only (scoped by default) */
  button {
    width: 50px;
    height: 50px;
    background-color: rgba(53, 63, 74, 0.5);
    border-radius: 14px;
    padding: 10px;
  }

  img {
    width: 100%;
    height: 100%;
  }
</style>