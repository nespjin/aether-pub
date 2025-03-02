import { KeyboardArrowDown } from "@mui/icons-material";
import { Checkbox, FormControlLabel, FormGroup } from "@mui/material";
import { useState } from "react";

interface ConditionConfig {
  keyword: string;
  name: string;
}

const PLATFORM_CONDITION_CONFIGS: ConditionConfig[] = [
  {
    keyword: "platform:android",
    name: "Android",
  },
  {
    keyword: "platform:ios",
    name: "iOS",
  },
  {
    keyword: "platform:linux",
    name: "Linux",
  },
  {
    keyword: "platform:macos",
    name: "macOS",
  },
  {
    keyword: "platform:web",
    name: "Web",
  },
];

const SDK_CONDITION_CONFIGS: ConditionConfig[] = [
  {
    keyword: "sdk:dart",
    name: "Dart",
  },
  {
    keyword: "sdk:flutter",
    name: "Flutter",
  },
];

const LICENSE_CONDITION_CONFIGS: ConditionConfig[] = [
  {
    keyword: "license:osi-approved",
    name: "OSI approved",
  },
];

const ADVANCED_CONDITION_CONFIGS: ConditionConfig[] = [
  {
    keyword: "is:flutter-favorite",
    name: "Flutter Favorite",
  },
  {
    keyword: "show:unlisted",
    name: "Include unlisted",
  },
  {
    keyword: "has:screenshot",
    name: "Has screenshot",
  },
  {
    keyword: "is:dart3-compatible",
    name: "Dart 3 compatible",
  },
  {
    keyword: "is:plugin",
    name: "Flutter plugin",
  },
  {
    keyword: "is:wasm-ready",
    name: "WASM ready",
  },
];

interface ConditionGroupProps {
  title: string;
  conditionConfigs: ConditionConfig[];
  onSelectChanged?: (config: ConditionConfig, isSelected: boolean) => void;
}

function ConditionGroup({
  title,
  conditionConfigs,
  onSelectChanged,
}: ConditionGroupProps): React.ReactNode {
  const [isShow, setShow] = useState(true);

  const arrowClassName = `
      h-7
      w-5
      ml-2
      transition-transform duration-300
      ${isShow ? "rotate-180" : ""}
      ${isShow ? "" : "-translate-x-1"}
      `;

  return (
    <div>
      <div
        className="h-10 w-full min-w-50 flex flex-row 
            items-center justify-between 
            cursor-pointer"
        onClick={() => setShow(!isShow)}
      >
        {title}
        <div className={arrowClassName}>
          <KeyboardArrowDown fontSize="medium" />
        </div>
      </div>
      {isShow && (
        <FormGroup>
          {conditionConfigs.map((config) => {
            return (
              <FormControlLabel
                className="hover:bg-gray-50 hover:rounded-sm p-0"
                control={
                  <Checkbox
                    onChange={(_, s) => {
                      onSelectChanged && onSelectChanged(config, s);
                    }}
                  />
                }
                label={config.name}
              />
            );
          })}
        </FormGroup>
      )}
    </div>
  );
}

export {
  PLATFORM_CONDITION_CONFIGS,
  SDK_CONDITION_CONFIGS,
  LICENSE_CONDITION_CONFIGS,
  ADVANCED_CONDITION_CONFIGS,
  ConditionGroup,
};

export type { ConditionConfig };
