export interface PackageBadgeProps {
  icon?: string;
  text: string;
}

export function PackageBadge({
  icon,
  text,
}: PackageBadgeProps): React.ReactNode {
  return (
    <div
      className="flex items-center gap-1 py-0.5 px-2
        border-1 border-blue-800 rounded-full text-blue-800"
    >
      {icon && <img src={icon} alt={text} className="w-3 h-3" />}
      <span className="text-xs">{text}</span>
    </div>
  );
}
