import icSearch from "~/assets/icons/ic_search.png";

export interface SearchViewProps {
  text: string;
  size: "large" | "small";
  width?: string;
  onTextChanged?: (text: string) => void;
}

export function SearchView({
  text,
  size,
  width,
  onTextChanged,
}: SearchViewProps) {
  const className = `
      ${size === "large" ? "h-16" : "h-12"}
      max-w-3xl
      ${size === "large" ? "text-2xl" : "text-x"}
      ${size === "large" ? "py-5" : "py-3"}
      ${size === "large" ? "px-6" : "px-5"}
      font-medium
      rounded-full
      text-white
      flex flex-row justify-start items-center
      `;
  return (
    <div className={className} style={{ width: width, background: "#35404d" }}>
      <img className="size-4 opacity-50" alt="Search" src={icSearch} />
      <input
        className="focus:outline-none w-full pl-2"
        type="text"
        value={text}
        onChange={(e) => onTextChanged && onTextChanged(e.target.value)}
        placeholder="Search packages"
      />
    </div>
  );
}
