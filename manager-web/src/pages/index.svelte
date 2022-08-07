<script>
    import {metatags, url} from '@roxi/routify'

    metatags.title = 'Container management service'
    metatags.description = 'container management service'
</script>

<style>
    .welcome {
        text-align: center;
    }
</style>

<div class="welcome">
    <h3>Welcome to the container management service</h3>
    <h4>
        <a href={$url('./containers')}>
            View containers
        </a>
    </h4>
</div>
