import React from "react";
import { invoke } from "@tauri-apps/api";
import { Nav } from "./components/nav";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
import {
  BaseDirectory,
  readBinaryFile,
  writeBinaryFile,
} from "@tauri-apps/api/fs";

function App() {
  // invoke("testBundle", { data:  });

  return (
    <div>
      <Nav />
      <div className="flex items-center justify-center h-screen px-4 py-6 bg-gray-800">
        <button
          className="px-2 py-3 text-3xl text-white rounded-lg bg-cyan-500"
          onClick={async () => {
            // const selected = await open({
            //   directory: true,
            //   multiple: false,
            //   defaultPath: await appDir(),
            // });

            // if (Array.isArray(selected) || selected == null) console.log("you can't do that, L");
            // else {
            //   console.log("selected dir: " + selected);
            // }

            invoke("send_file", { filePath: "/Users/nebudev14/Desktop/savonius.SLDPRT" })
            // const contents = await readBinaryFile("savonius.SLDPRT", {
            //   dir: BaseDirectory.Desktop,
            // }).then((data) => {
            //   console.log(typeof data);

            // });
          }}
        >
          Create a directory
        </button>
      </div>
    </div>
  );
}

export default App;
