import init, {
    remove_exif,
    detect_image_type,
    detect_image_extension,
    supported_formats,
    supported_mime_types
} from "./merfix.js";

async function setup() {
    await init(); // Load WebAssembly module

    console.log('>> supported formats', supported_formats());
    console.log('>> supported mime types', supported_mime_types());

    document.getElementById("process-btn").addEventListener("click", async () => {
        const fileInput = document.getElementById("file-input");
        if (!fileInput.files.length) {
            return alert("Select an image");
        }

        const file = fileInput.files[0];
        const fileName = file.name;
        const fileExtension = fileName.split('.').pop();

        console.log('>> file name', fileName);
        console.log('>> file extension', fileExtension);

        const arrayBuffer = await file.arrayBuffer();
        const raw = new Uint8Array(arrayBuffer);
        const result = remove_exif(raw, fileExtension);

        const detectedType = detect_image_type(raw);
        console.log('>> detected type', detectedType);

        const detectedExtension = detect_image_extension(raw);
        console.log('>> detected extension', detectedExtension);

        if (result.is_error()) {
            console.error(">> error", result.get_error());
        }

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

setup().then(r => console.log('>> setup ready'));
