import type React from "react";
import { MenuBar } from "~/components/MenuBar";
import heroBgStatic from "~/assets/images/hero-bg-static.svg";
import { SearchView } from "~/components/SearchView";
import {
  ADVANCED_CONDITION_CONFIGS,
  ConditionGroup,
  LICENSE_CONDITION_CONFIGS,
  PLATFORM_CONDITION_CONFIGS,
  SDK_CONDITION_CONFIGS,
  type ConditionConfig,
} from "./conditions";
import { useState } from "react";
import { ORDER_CONFIGS, PackageListView } from "./package-list-view";
import { List } from "@mui/material";
import { PackageListItem } from "./package-list-item";
import { Footer } from "~/components/Footer";

export function PackageList(): React.ReactNode {
  const [keywords, setKeywords] = useState("");

  const [conditionKeywords, setConditionKeywords] = useState<string[]>([]);

  const onConditionConfigSelectChanged = (
    config: ConditionConfig,
    isSelected: boolean
  ) => {
    if (isSelected && !keywords.includes(config.keyword)) {
      setKeywords((prev) => {
        let prevValue = prev;
        const len = prevValue.length;
        if (len > 0 && prev.charCodeAt(len - 1) !== " ".charCodeAt(0)) {
          prevValue += " ";
        }
        return prevValue + config.keyword;
      });
    } else if (!isSelected && keywords.includes(config.keyword)) {
      setKeywords((prev) => prev.replace(config.keyword, ""));
    }
  };

  return (
    <div>
      <MenuBar />
      <div
        className="h-23 flex px-5 flex-row items-center justify-center bg-cover bg-center bg-no-repeat"
        style={{
          backgroundImage: heroBgStatic,
          background: "#132030",
          color: "#8d9399",
        }}
      >
        <SearchView
          text={keywords}
          size="small"
          width="100%"
          onTextChanged={setKeywords}
        />
      </div>
      {/* Content */}
      <div className="w-full flex flex-row justify-center">
        <div className="px-15 max-w-6xl w-full flex md:flex-row flex-col justify-between ">
          {/* Conditions */}
          <div>
            <ConditionGroup
              title="Platforms"
              onSelectChanged={onConditionConfigSelectChanged}
              conditionConfigs={PLATFORM_CONDITION_CONFIGS}
            />

            <ConditionGroup
              title="SDKs"
              onSelectChanged={onConditionConfigSelectChanged}
              conditionConfigs={SDK_CONDITION_CONFIGS}
            />

            <ConditionGroup
              title="License"
              onSelectChanged={onConditionConfigSelectChanged}
              conditionConfigs={LICENSE_CONDITION_CONFIGS}
            />

            <ConditionGroup
              title="Advanced"
              onSelectChanged={onConditionConfigSelectChanged}
              conditionConfigs={ADVANCED_CONDITION_CONFIGS}
            />
          </div>
          <PackageListView
            totalCount={1000}
            totalPages={100}
            currentPage={1}
            orderConfig={ORDER_CONFIGS[0]}
            items={[]}
          />
        </div>
      </div>
      <Footer />
    </div>
  );
}
