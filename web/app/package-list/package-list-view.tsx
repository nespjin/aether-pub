import { List, Pagination } from "@mui/material";
import {
  PackageListItem,
  type PackageListItemProps,
} from "./package-list-item";

export interface OrderConfig {
  name: string;
}

export const ORDER_CONFIGS: OrderConfig[] = [
  {
    name: "Listing Relevance",
  },
  {
    name: "Recently Updated",
  },
  {
    name: "Newest Package",
  },
  {
    name: "Downloads",
  },
];

export interface PackageListViewProps {
  totalCount: number;
  totalPages: number;
  currentPage: number;
  orderConfig: OrderConfig;
  items: PackageListItemProps[];
}

export function PackageListView({
  totalCount,
  totalPages,
  currentPage,
  orderConfig,
  items,
}: PackageListViewProps): React.ReactNode {
  return (
    <div className="w-full flex flex-col items-center justify-start pt-2 pb-10">
      <List sx={{ width: "100%" }}>
        {new Array(10).fill(0).map((_, index) => {
          return (
            <PackageListItem
              key={index}
              name="flutter_hooks"
              description="A set of Flutter hooks for simplifying common use cases."
              topics={["hooks", "flutter"]}
              downloads={100000}
              version="1.0.0"
              license="MIT"
              isFlutterFavorite={true}
              isDart3Compatible={true}
              sdks={["flutter"]}
              platforms={["android", "ios", "web"]}
            />
          );
        })}
      </List>
      <Pagination
        className="mt-10"
        count={10}
        variant="outlined"
        shape="rounded"
      />
    </div>
  );
}
