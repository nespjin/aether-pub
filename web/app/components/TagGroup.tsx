export interface TagGroupProps {
  title: string;
  tags: string[];
}

export function TagGroup({ title, tags }: TagGroupProps): React.ReactNode {
  return (
    <div className="flex flex-row">
      <div className="text-blue-500 text-xs font-semibold uppercase px-2 py-1 bg-blue-100 border-r-1 border-blue-300">
        {title}
      </div>
      <div className="flex flex-row flex-wrap">
        {tags.map((tag) => (
          <div className="text-blue-500 text-xs font-normal hover:underline cursor-pointer uppercase px-2 py-1 bg-blue-100">
            {tag}
          </div>
        ))}
      </div>
    </div>
  );
}
