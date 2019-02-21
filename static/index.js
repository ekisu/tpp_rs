function onPageLoaded() {
    var app = new Vue({
        el: "#main_container",
        data: {
            renderer_data: {
                last_inputs: [],
                last_vote_system: null,
                last_vote_system_percentage: null,
                last_vote_system_partial_results: null,
                last_vote_system_elapsed_time: null
            }
        },
        filters: {
            whenNull: function(value, or) {
                return value !== null ? value : or;
            }
        }
    });

    const updateData = async () => {
        let resp = await fetch("/data");

        app.renderer_data = await resp.json();
    }

    setInterval(updateData, 1000);
}