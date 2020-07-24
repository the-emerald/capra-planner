<template>
    <div>
        <b-modal id="about-modal" title="About" hide-footer>
            <b-container>
                <!-- Logo and name -->
                <b-row>
                    <b-col sm="4">
                        <b-img src="../static/1f410.svg"></b-img>
                    </b-col>
                    <b-col>
                        <p><b>Capra Dive Planner v0.1.0</b></p>
                        <p>Copyright © 2020 Anson Cheung</p>
                        <a href="#" @click="repo">View source</a>
                    </b-col>
                </b-row>
                <hr>
                <b-row>
                    <b-col>
                        <p>Logo sourced from Twemoji.</p>
                    </b-col>
                </b-row>
            </b-container>
            <div class="modal-footer">
                <b-button @click="$bvModal.hide('about-modal')" variant="primary">OK</b-button>
            </div>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";

    @Component({})
    export default class SurfaceIntervalModal extends Vue {
        repo() {
            window.open("https://github.com/the-emerald/capra-planner/", '_blank');
        }
    }
</script>

<style scoped>

</style>