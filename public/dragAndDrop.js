// Import Sortable from a CDN
const script = document.createElement('script');
script.src = "https://cdnjs.cloudflare.com/ajax/libs/Sortable/1.14.0/Sortable.min.js";
script.onload = () => {
    // Initialize Sortable after the script is loaded
    const listExample = document.querySelector("#list-example");

    if (listExample) {
        Sortable.create(listExample, {
            animation: 150,
            dragClass: "!border-0"
        });
    }
};
document.head.appendChild(script);