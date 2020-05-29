<template>
    <div class="test">
        <Navbar></Navbar>
        <br>
        <p>Testing.</p>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import Navbar from "@/components/Navbar.vue";

    @Component({
        name: "Test",
        components: {
            Navbar,
        }
    })
    export default class Test extends Vue {

    }
</script>

<style scoped>

</style>