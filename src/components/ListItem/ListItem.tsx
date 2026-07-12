import { Delete } from "react-feather";

export type ListItemProps = {
  label: string;
  onClick?: () => void;
  onDelete?: () => void;
  active?: boolean;
  deleteItemActive?: boolean;
};

type ListItemButtonProps = React.PropsWithChildren & {
  className?: string;
  onClick?: () => void;
  active?: boolean;
}

const ListItemButtonStyle = "flex-1 min-w-0 h-[24px] border-0 rounded-sm text-left overflow-hidden";

const ActiveListItemButtonStyle = "bg-[#0a84ff]";

const ListItemButton = ({ onClick, active, className, ...props }: ListItemButtonProps) => {
  const buttonStyle = [ListItemButtonStyle, className];

  if (active) {
    buttonStyle.push(ActiveListItemButtonStyle);
  }

  return (
    <button
      className={buttonStyle.join(" ")}
      onClick={onClick}
      {...props}
    >
      {props.children}
    </button>
  );
};

export const ListItem = ({ label, onClick, active, deleteItemActive, onDelete, ...props }: ListItemProps) => (
  <div className="flex w-full justify-between items-center my-1 gap-1">
    <div className="flex min-w-0 flex-1 justify-left items-center">
      <ListItemButton onClick={onClick} className="hover:bg-[#0a84ff] px-2" active={active && !deleteItemActive} {...props}>
        <span className="block text-sm min-w-0 text-nowrap whitespace-nowrap text-ellipsis overflow-hidden">{label}</span>
      </ListItemButton>
    </div>

    <div className="shrink-0 flex justify-right items-center mr-2">
      <ListItemButton className="cursor-pointer px-1" onClick={onDelete} active={deleteItemActive}>
        <Delete className="w-4 h-4 m-auto" />
      </ListItemButton>
    </div>
  </div>
);
