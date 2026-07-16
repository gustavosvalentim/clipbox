import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Trash2 } from "react-feather";
import { ListItem } from "./components/ListItem";
import "./App.css";

type ClipboardItem = {
	hash: string;
	text: string;
};

type Clipboard = ClipboardItem[];

const MenuSeparator = () => (
	<div className="h-px my-[4px] mx-0 bg-[rgba(235,235,245,0.18)]" />
);

function App() {
	const [clipboard, setClipboard] = useState<Clipboard>([]);
	const [selectedItem, setSelectedItem] = useState<number | null>(null);
	const [isDeleteItemActive, setIsDeleteItemActive] = useState(false);

	const hide = useCallback(() => invoke("close"), []);

	const fetchClipboardHistory = useCallback(async () => {
		try {
			const clipboard = await invoke<ClipboardItem[]>("fetch_clipboard");
			setClipboard(clipboard);
		} catch (error) {
			console.error("Failed to get clipboard history", error);
		}
	}, []);

	const clearHistory = useCallback(async () => {
		try {
			await invoke("clear");
		} catch (error) {
			console.error("Failed to clear clipboard history", error);
		}
	}, []);

	const pasteFromSelection = useCallback(async (text: string) => {
		try {
			await invoke("paste", { text });
		} catch (error) {
			console.error("Failed to paste from selection", error);
		}
	}, []);

	const deleteItem = useCallback(async (text: string) => {
		await invoke("delete_item", { text });
		setSelectedItem((prev) => (prev && prev > 0 ? prev - 1 : null));
	}, []);

	const actionsMenuItems = [
		{ label: "Clear History", onClick: clearHistory, icon: Trash2 },
	];

	const clipboardMenuItems = useMemo(
		() =>
			clipboard.map((item, idx) => ({
				label: `${idx}. ${item.text}`,
				key: item.hash,
				onClick: () => pasteFromSelection(item.text),
				onDelete: () => deleteItem(item.text),
			})),
		[clipboard, pasteFromSelection, deleteItem],
	);

	const handleKeyDown = useCallback(
		(event: KeyboardEvent) => {
			const isValidItem = (itemIdx: number) =>
				itemIdx >= 0 && itemIdx < clipboard.length;

			let newSelectedItem = selectedItem;

			switch (event.key) {
				case "ArrowUp":
					event.preventDefault();
					newSelectedItem =
						selectedItem !== null ? selectedItem - 1 : clipboard.length;
					setIsDeleteItemActive(false);
					break;
				case "ArrowDown":
					event.preventDefault();
					newSelectedItem = selectedItem !== null ? selectedItem + 1 : 0;
					setIsDeleteItemActive(false);
					break;
				case "ArrowRight":
					event.preventDefault();
					if (!isDeleteItemActive) {
						setIsDeleteItemActive(true);
					}
					break;
				case "ArrowLeft":
					event.preventDefault();
					if (isDeleteItemActive) {
						setIsDeleteItemActive(false);
					}
					break;
				case "Enter": {
					event.preventDefault();

					if (selectedItem !== null && isValidItem(selectedItem)) {
						if (isDeleteItemActive) {
							deleteItem(clipboard[selectedItem].text);
							return;
						}

						pasteFromSelection(clipboard[selectedItem].text);
					}

					return;
				}
				case "Backspace":
				case "Delete":
					event.preventDefault();

					if (selectedItem !== null && isValidItem(selectedItem)) {
						deleteItem(clipboard[selectedItem].text);
					}

					break;
				case "Escape":
					event.preventDefault();
					hide();
					break;
				default:
					break;
			}

			if (newSelectedItem === null || !isValidItem(newSelectedItem)) {
				newSelectedItem = 0;
			}

			setSelectedItem(newSelectedItem);
		},
		[
			clipboard,
			isDeleteItemActive,
			selectedItem,
			pasteFromSelection,
			hide,
			deleteItem,
		],
	);

	const handleBlur = useCallback(() => {
		setSelectedItem(null);
	}, []);

	const handleFocus = useCallback(() => {
		fetchClipboardHistory();
	}, [fetchClipboardHistory]);

	useEffect(() => {
		const unlisten = listen<string>("clipboard-changed", async () => {
			const isVisible = await getCurrentWindow().isVisible();

			if (!isVisible) return;

			fetchClipboardHistory();
		});

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
	}, [handleKeyDown, handleBlur, handleFocus]);

	return (
		<div className="menu text-gray-100/80">
			<div className="menu__content">
				<div className="flex justify-between items-center mx-2">
					<div className="flex justify-left items-center">
						<span className="text-base font-bold">Klipo</span>
					</div>

					<div className="flex justify-right items-center">
						{actionsMenuItems.map((item) => (
							<button
								type="button"
								className="cursor-pointer p-1 rounded-md hover:bg-[#0a84ff]"
								onClick={item.onClick}
								key={item.label}
							>
								<item.icon className="w-4 h-4" />
							</button>
						))}
					</div>
				</div>

				<MenuSeparator />

				<div className="menu__history">
					{clipboardMenuItems.map((item, idx) => (
						<ListItem
							{...item}
							key={item.key}
							active={idx === selectedItem}
							deleteItemActive={idx === selectedItem && isDeleteItemActive}
						/>
					))}
				</div>
			</div>
		</div>
	);
}

export default App;
