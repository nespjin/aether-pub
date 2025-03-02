import type React from "react";

export interface TopicViewProps {
  text: string;
}

export function TopicView({ text }: TopicViewProps): React.ReactNode {
  return (
    <div className="text-blue-700 hover:underline cursor-pointer">{`#${text}`}</div>
  );
}
