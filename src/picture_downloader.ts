import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

let buttonInputFile;
let buttonOutputPath;
let buttonStart;
let inputFile: any;
let outputPath: any;
let divOutput: any;

async function get_input_file() {
  var selected = await open({
    multiple: false,
    filters: [{
      name: 'CSV',
      extensions: ['csv']
    }]
  });

  if (selected === null) {
    return "";
  } else {
    return selected;
  }
}

async function get_output_file() {
  const filePath = await open({
    directory: true
  });

  if (filePath === null) {
    return "";
  } else {
    return filePath;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  buttonInputFile = document.querySelector("#buttonInputFile");
  buttonOutputPath = document.querySelector("#buttonOutputPath");
  buttonStart = document.querySelector("#buttonStart");
  inputFile = document.querySelector("#inputFile");
  outputPath = document.querySelector("#outputPath");
  divOutput = document.querySelector("#output");

  // Select Input File
  buttonInputFile!.addEventListener("click", async (e) => {
    e.preventDefault();
    inputFile.value = await get_input_file();
  });
  // Select Output File
  buttonOutputPath!.addEventListener("click", async (e) => {
    e.preventDefault();
    outputPath.value = await get_output_file();
  });

  // Start
  buttonStart!.addEventListener("click", async (e) => {
    e.preventDefault();
    if (outputPath.value == "") {
      divOutput.innerHTML = "Kein Ziel Pfad";
      return
    }
    divOutput.innerHTML = "Verarbeite...";
    var resp = await invoke("download_files", {
      inputfile: inputFile.value, outputpath: outputPath.value + "\\"
    });
    divOutput.innerHTML = resp;
  });
});
