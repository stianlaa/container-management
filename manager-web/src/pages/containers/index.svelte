<script>
    import {url} from '@roxi/routify'

    let exampleData = {
        "example1": {
            name: "example name 1",
            description: "description1",
            status: "Active",
        },
        "example2": {
            name: "example name 2",
            description: "description2",
            status: "Inactive",
        }
    }
</script>

<style>
    .service-btn {
        margin: 20px 0 20px 0;
    }
</style>

<div style="width: 100%; text-align: center; margin-top: 4rem;">
    <h4>Containers:</h4>
    {#each Object.entries(exampleData) as [serviceName, serviceStatus]}
        <a href={$url("./:serviceName", { serviceName })}
           class={`flex-item-grow service-btn btn-large ${serviceStatus.status === "Active" ? "blue-grey" : "yellow darken-4"}`}>
            <i class="material-icons left">{serviceStatus.status === "Active" ? "info" : "warning"}</i>
            {serviceName}
        </a>
        <br/>
    {/each}

</div>
