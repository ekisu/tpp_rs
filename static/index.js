function onPageLoaded() {
    var app = new Vue({
        el: "#last_commands",
        data: {
            last_commands: []
        }
    });

    const updateLastCommands = async () => {
        let resp = await fetch("/last_commands");

        app.last_commands = await resp.json();
    }

    setInterval(updateLastCommands, 1000);
}