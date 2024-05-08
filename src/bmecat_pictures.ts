import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { save } from '@tauri-apps/api/dialog';

let buttonInputFile;
let buttonOutputFile;
let buttonStart;
let inputFile: any;
let outputFile: any;
let radioPicturePreset: any;
let divOutput: any;

async function get_input_file() {
  var selected = await open({
    multiple: false,
    filters: [{
      name: 'BMEcat',
      extensions: ['xml']
    }]
  });

  if (selected === null) {
    return "";
  } else {
    return selected;
  }
}

async function get_output_file() {
  const filePath = await save({
    filters: [{
      name: 'CSV',
      extensions: ['csv']
    }]
  });

  if (filePath === null) {
    return "";
  } else {
    return filePath;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  buttonInputFile = document.querySelector("#buttonInputFile");
  buttonOutputFile = document.querySelector("#buttonOutputFile");
  buttonStart = document.querySelector("#buttonStart");
  inputFile = document.querySelector("#inputFile");
  outputFile = document.querySelector("#outputFile");
  radioPicturePreset = document.getElementsByName('picturePreset');
  divOutput = document.querySelector("#output");

  // Select Input File
  buttonInputFile!.addEventListener("click", async (e) => {
    e.preventDefault();
    inputFile.value = await get_input_file();
  });
  // Select Output File
  buttonOutputFile!.addEventListener("click", async (e) => {
    e.preventDefault();
    outputFile.value = await get_output_file();
  });

  // Start
  buttonStart!.addEventListener("click", async (e) => {
    e.preventDefault();
    var preset = 0;
    if (radioPicturePreset[0].checked) {
      preset = 1;
    }
    if (radioPicturePreset[1].checked) {
      preset = 2;
    }
    divOutput.innerHTML = "Verarbeite...";
    var resp = await invoke("bmecat_to_picture_csv", { inputpath: inputFile.value, outputpath: outputFile.value, preset: preset });
    divOutput.innerHTML = resp;
  });
});
