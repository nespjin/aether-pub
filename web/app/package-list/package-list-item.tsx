import { ContentCopy, CopyAll } from "@mui/icons-material";
import { PackageBadge } from "~/components/PackageBadge";
import { TagGroup } from "~/components/TagGroup";
import { TopicView } from "~/components/TopicView";

export interface PackageListItemProps {
  name: string;
  description: string;
  topics: string[];
  downloads: number;
  version: string;
  license: string;
  isFlutterFavorite: boolean;
  isDart3Compatible: boolean;
  sdks: string[];
  platforms: string[];
}

export function PackageListItem({
  name,
  description,
  topics,
  downloads,
  version,
  license,
  isFlutterFavorite,
  isDart3Compatible,
  sdks,
  platforms,
}: PackageListItemProps): React.ReactNode {
  return (
    <div className="p-5 flex flex-col hover:bg-gray-50 hover:rounded-sm">
      <div className="flex flex-row justify-items-stretch justify-between">
        <div className="text-xl inline-flex gap-3">
          <div
            className="font-semibold hover:underline cursor-pointer"
            style={{ color: "#1967d2" }}
          >
            {name}
          </div>
          <div className="text-gray-400 hover:text-gray-600 hover:cursor-pointer">
            <ContentCopy fontSize="small" />
          </div>
        </div>
        <div className="flex flex-col items-center justify-center">
          <div className="text-xl" style={{ color: "#1967d2" }}>
            {downloads}
          </div>
          <div className="mt-1 text-xs text-black opacity-55">DOWNLOADS</div>
        </div>
      </div>
      <div className="flex flex-row font-light ">
        <div>{description}</div>
        &nbsp;
        {topics.map((topic) => (
          <div className="mr-2">
            <TopicView key={topic} text={topic} />
          </div>
        ))}
      </div>
      <div className="flex flex-row gap-2 font-light text-xs mt-4 items-center">
        <div className="inline-flex">
          v
          <div className="text-blue-600 hover:underline cursor-pointer">
            {version}
          </div>
        </div>
        <div>{license}</div>
        <PackageBadge
          text="Flutter Favorite"
          icon="https://pub.dev/static/hash-kjnuqhji/img/flutter-logo-32x32.png"
        />
        <PackageBadge text="Dart 3 compatible" />
      </div>
      <div className="flex flex-row gap-4 font-light text-sm mt-2">
        <TagGroup title="SDk" tags={["dart", "flutter"]} />
        <TagGroup
          title="platform"
          tags={["android", "web", "windows", "macos"]}
        />
      </div>
    </div>
  );
}
