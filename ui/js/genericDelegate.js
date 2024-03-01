const invoke = window.__TAURI__.invoke;

async function query_generic_details(generic_name) {
    return invoke('generic_details', { name: generic_name });
}

async function create_ticket(data) { 
    return invoke('create_ticket', { data: data } );
}

const submitButton = document.getElementById("submit");
const queryOption = document.getElementById("querySelect");

submitButton.addEventListener("click", async function (_) {
    const generic_name = document.getElementById("generic").value;
    if (generic_name === "") {
        submitButton.classList.add("bg-red-500", "border-red-500", "text-white");
        submitButton.innerHTML = "Enter a name!";
        await setTimeout(async function () {
            submitButton.classList.remove("bg-red-500", "border-red-500", "text-white");
            submitButton.innerHTML = "Search";
        }, 1000);
        return;
    } else {
        switch (queryOption.value) {
            case "LNG":
                submitButton.disabled = true;
                submitButton.innerHTML = "Searching...";

                const details = await query_generic_details(generic_name);
                if (details !== null && details !== undefined) {
                    console.log(details);
                    const ticket = await create_ticket(details);
                    window.location.replace("generics_details.html?ticket=" + ticket);

                    submitButton.disabled = false;
                    submitButton.innerHTML = "Redirecting...";
                } else {
                    submitButton.classList.add("bg-red-500", "border-red-500", "text-white");
                    submitButton.innerHTML = "No results found!";
                    await setTimeout(async function () {
                        submitButton.classList.remove("bg-red-500", "border-red-500", "text-white");
                        submitButton.innerHTML = "Search";
                    }, 1000);
                }
                break;
        }
    }
})