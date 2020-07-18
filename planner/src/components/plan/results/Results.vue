<template>
    <div>
        <b-card-group>
            <b-card no-body>
                <b-card-header>
                    <b-container class="p-0">
                        <b-form-row>
                            <b-col sm="6" style="margin-top: 0.2rem">
                                Results
                            </b-col>
                            <!--UI buttons-->

                            <!--Reorder-->
                            <b-col>
                                <b-button block
                                          class="float-right" id="rearrange"
                                          size="sm"
                                          title="Reorder">
                                    <b-icon-list-ol></b-icon-list-ol>
                                </b-button>
                            </b-col>

                        </b-form-row>
                    </b-container>

                    <!--Relevant modals-->
                </b-card-header>
                <b-container>
                    <p>Reserved for showing dive results bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg bleg</p>
                </b-container>
            </b-card>
        </b-card-group>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";

    @Component({})
    export default class Results extends Vue {

    }


</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>