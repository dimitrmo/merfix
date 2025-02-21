import init, { remove_exif } from "./merfix.js";

async function setup() {
    await init(); // Load WebAssembly module

    document.getElementById("process-btn").addEventListener("click", async () => {
        const fileInput = document.getElementById("file-input");
        if (!fileInput.files.length) return alert("Select an image");

        const file = fileInput.files[0];
        const arrayBuffer = await file.arrayBuffer();
        const processedBytes = remove_exif(new Uint8Array(arrayBuffer));

        // Create downloadable image
        const blob = new Blob([processedBytes], { type: file.type });
        const url = URL.createObjectURL(blob);

        const link = document.getElementById("download-link");
        link.href = url;
        link.download = "image_no_exif.jpg";
        link.style.display = "block";
        link.textContent = "Download Image";
    });
}

setup().then(r => console.log('setup ready'));
