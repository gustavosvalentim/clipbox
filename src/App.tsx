import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from '@tauri-apps/api/window';
import "./App.css";

type ClipboardItem = {
  text: string;
};

type Clipboard = ClipboardItem[];

function App() {
  const [clipboard, setClipboard] = useState<Clipboard>([]);

  const hideWindow = () => getCurrentWindow().hide();

  const getClipboardHistory = () => {
    invoke<ClipboardItem[]>("list_clipboard_items")
      .then((history: Clipboard) => {
        console.log(history);
        setClipboard(history);
      });
  }

  const clearHistory = async () => {
    try {
      await invoke("clear_clipboard_items");
      getClipboardHistory();
    } catch (error) {
      console.error("Failed to clear clipboard history", error);
    }
  }

  const pasteFromSelection = async (text: string) => {
    try {
      await invoke("paste_from_selection", { text });
      getClipboardHistory();
    } catch (error) {
      console.error("Failed to paste from selection", error);
    }
  }

  useEffect(() => {
    const unlisten = listen<string>("clipboard-changed", getClipboardHistory);

    return () => {
      unlisten
        .then((unlisten) => unlisten());
    }
  }, []);

  return (
    <div className="clipboard-container">
      <div className="topbar">
        <p className="title">History</p>
      </div>

      <div className="clipboard">
        <div className="clipboard__history">
          {clipboard.map((item) => (
            <div
              className="clipboard__item"
              key={item.text}
              onClick={() => pasteFromSelection(item.text)}
            >
              <span>{item.text}</span>
            </div>
          ))}
        </div>

        <div className="clipboard__menu">
          <div className="clipboard__item" onClick={clearHistory}>
            <span>Clear</span>
          </div>
        
          <div className="clipboard__item" onClick={hideWindow}>
            <span>Quit</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
