<script>
    import Button from "$lib/components/Button.svelte";

    function demoFunction() {
        alert("button works")
    }

</script>

<main></main>

<style>
    main {
        display: flex;
        justify-content: start;
        align-items: start;
        flex-direction: column;
        width: 50%;
        height: 100dvh;
        background-color: orange;
    }
</style>
<Button iconPath="/src/lib/assets/icons/menu.svg" onClick={demoFunction}></Button>