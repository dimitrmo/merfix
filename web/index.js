import init, { remove_exif, supported_formats } from "./merfix.js";

async function setup() {
    await init(); // Load WebAssembly module

    console.log('>> supported formats', supported_formats());

    document.getElementById("process-btn").addEventListener("click", async () => {
        const fileInput = document.getElementById("file-input");
        if (!fileInput.files.length) return alert("Select an image");

        const file = fileInput.files[0];
        const fileName = file.name;
        const fileExtension = fileName.split('.').pop();

        const arrayBuffer = await file.arrayBuffer();
        const result = remove_exif(new Uint8Array(arrayBuffer), fileExtension);
        const processedBytes = result.get_data();

        // Create downloadable image
        const blob = new Blob([processedBytes], { type: file.type });
        const url = URL.createObjectURL(blob);

        const link = document.getElementById("download-link");
        link.href = url;
        link.download = `image_no_exif.${fileExtension}`;
        link.style.display = "block";
        link.textContent = "Download Image";
    });
}

setup().then(r => console.log('setup ready'));
