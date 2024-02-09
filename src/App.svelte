<script lang="ts">
    const exit_application = () => {
        close();
    };
</script>

<main>
    <ul class="button-list">
        <li>
            <button>Создать сервер</button>
        </li>
        <li>
            <button>Подключиться к серверу</button>
        </li>
        <li>
            <button>Настройки</button>
        </li>
        <li>
            <button on:click={exit_application}>Выход</button>
        </li>
    </ul>
</main>

<style>
    :global(body) {
        background-image: url('./assets/main_menu_background.jpg');
        background-size: cover;
        background-position: center;
        min-height: 100vh;
        min-width: 100vw;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    .button-list {
        list-style: none;
        margin: 100px 0 20px;
        padding: 50px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    li {
        margin-bottom: 10px;
    }

    button {
        padding: 10px;
        width: 250px;
        font-size: 16px;
        background-color: rgba(22, 21, 21, 0.9);
        cursor: pointer;
    }
</style>
