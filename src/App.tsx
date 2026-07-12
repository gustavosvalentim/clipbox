import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Trash2, Delete } from "react-feather";
import "./App.css";

type ClipboardItem = {
	hash: string;
	text: string;
};

type Clipboard = ClipboardItem[];

type MenuItemProps = {
	label: string;
	onClick: () => void;
  onDelete: () => void;
	active?: boolean;
};

const MenuItem = ({ label, onClick, active, onDelete, ...props }: MenuItemProps) => (
  <div className="flex w-full justify-between items-center">
    <div className="flex min-w-0 flex-1 justify-left items-center">
      <button
        className="flex-1 min-w-0 h-[24px] border-0 rounded-sm text-left overflow-hidden hover:bg-[#0a84ff] px-2"
        onClick={onClick}
        {...props}
      >
        <span className="block min-w-0 text-nowrap whitespace-nowrap text-ellipsis overflow-hidden">{label}</span>
      </button>
    </div>

    <div className="shrink-0 flex justify-right items-center">
      <div className="p-2 cursor-pointer" onClick={onDelete}>
        <Delete className="w-6 h-6 p-1 rounded-md hover:bg-[#0a84ff]" />
      </div>
    </div>
  </div>
);

const MenuSeparator = () => <div className="h-px my-[4px] mx-0 bg-[rgba(235,235,245,0.18)]" />;

function App() {
	const [clipboard, setClipboard] = useState<Clipboard>([]);
	const [selectedItem, setSelectedItem] = useState<number | null>(null);

	const hideClipbox = useCallback(() => invoke("hide_clipbox"), []);

	const fetchClipboardHistory = useCallback(async () => {
		try {
			const clipboard = await invoke<ClipboardItem[]>("list_clipboard_items");
			setClipboard(clipboard);
		} catch (error) {
			console.error("Failed to get clipboard history", error);
		}
	}, []);

	const clearHistory = useCallback(async () => {
		try {
			await invoke("clear_clipboard_items");
			fetchClipboardHistory();
		} catch (error) {
			console.error("Failed to clear clipboard history", error);
		}
	}, [fetchClipboardHistory]);

	const pasteFromSelection = useCallback(
		async (text: string) => {
			try {
				await invoke("paste_from_selection", { text });
				fetchClipboardHistory();
			} catch (error) {
				console.error("Failed to paste from selection", error);
			}
		},
		[fetchClipboardHistory],
	);

  const deleteItem = useCallback((text: string) => invoke("delete_item", { text }), []);

	const actionsMenuItems = [
    { "label": "Clear History", "onClick": clearHistory, icon: Trash2 },
	];

	const clipboardMenuItems = useMemo(
		() =>
			clipboard.map((item) => ({
				label: `${item.text}`,
				key: item.hash,
				onClick: () => pasteFromSelection(item.text),
        onDelete: () => deleteItem(item.text),
			})),
		[clipboard, pasteFromSelection],
	);

	const handleKeyDown = useCallback(
		(event: KeyboardEvent) => {
			let newSelectedItem = selectedItem;
			switch (event.key) {
				case "ArrowUp":
					event.preventDefault();
					newSelectedItem =
						selectedItem !== null ? selectedItem - 1 : clipboard.length;
					break;
				case "ArrowDown":
					event.preventDefault();
					newSelectedItem = selectedItem !== null ? selectedItem + 1 : 0;
					break;
				case "Enter":
					event.preventDefault();
					if (
						selectedItem !== null &&
						selectedItem >= 0 &&
						selectedItem < clipboard.length
					) {
						pasteFromSelection(clipboard[selectedItem].text);
					}
					break;
				case "Escape":
					event.preventDefault();
					hideClipbox();
					break;
				default:
					break;
			}

			if (
				newSelectedItem !== null &&
				(newSelectedItem < 0 || newSelectedItem >= clipboard.length)
			) {
				newSelectedItem = 0;
			}

			setSelectedItem(newSelectedItem);
		},
		[clipboard, selectedItem, pasteFromSelection, hideClipbox],
	);

	const handleBlur = useCallback(() => {
		setSelectedItem(null);
	}, []);

	const handleFocus = useCallback(() => {
		fetchClipboardHistory();
	}, [fetchClipboardHistory]);

	useEffect(() => {
		const unlisten = listen<string>("clipboard-changed", fetchClipboardHistory);

		return () => {
			unlisten.then((unlisten) => unlisten());
		};
	}, [fetchClipboardHistory]);

	useEffect(() => {
		window.addEventListener("keydown", handleKeyDown);
		window.addEventListener("focus", handleFocus);
		window.addEventListener("blur", handleBlur);

		return () => {
			window.removeEventListener("keydown", handleKeyDown);
			window.removeEventListener("focus", handleFocus);
			window.removeEventListener("blur", handleBlur);
		};
	}, [handleKeyDown, handleFocus, handleBlur]);

	return (
		<div className="menu">
			<div className="menu__content">
        <div className="flex justify-between items-center">
          <div className="flex justify-left items-center px-2">
            <span className="font-size-10 font-bold">Clipbox</span>
          </div>

          <div className="flex justify-right items-center">
            {actionsMenuItems.map((item) => (
              <div className="p-2 cursor-pointer" onClick={item.onClick}>
                <item.icon className="w-6 h-6 p-1 rounded-md hover:bg-[#0a84ff]" />
              </div>
            ))}
          </div>
        </div>

				<MenuSeparator />

				<div className="menu__history">
					{clipboardMenuItems.map((item, idx) => (
						<MenuItem {...item} active={idx === selectedItem} key={item.key} />
					))}
				</div>
			</div>
		</div>
	);
}

export default App;
