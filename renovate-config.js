module.exports = {
  // Bot settings
  platform: "github",
  autodiscover: false,
  onboarding: false,
  requireConfig: "optional",
  prHourlyLimit: 10,
  prConcurrentLimit: 20,
  semanticCommits: "enabled",

  // Vulnerability scanning
  osvVulnerabilityAlerts: true,
  vulnerabilityAlerts: {
    labels: ["security", "high-priority"],
    commitMessageSuffix: "[SECURITY]"
  },

  // Dependency settings
  extends: [
    "config:best-practices"
  ],
  timezone: "Europe/Copenhagen",
  labels: ["deps-bot"],

  nix: {
    enabled: true
  },

  customManagers: [
    {
      description: "Update rust-toolchain channel via GitHub releases",
      customType: "regex",
      fileMatch: [
        "(^|/)rust-toolchain(\\.toml)?$"
      ],
      matchStrings: [
        "channel\\s*=\\s*\"(?<currentValue>[0-9.]+)\""
      ],
      depNameTemplate: "rust-lang/rust",
      datasourceTemplate: "github-releases",
      versioningTemplate: "semver"
    }
  ],

  packageRules: [
    {
      "description": "Wait 5 days before creating PRs for standard updates to ensure stability",
      "matchUpdateTypes": ["major", "minor", "patch"],
      "minimumReleaseAge": "5 days",
    },
    {
      "description": "Group all minor and patch Rust crate updates together",
      "matchManagers": ["cargo"],
      "matchUpdateTypes": ["minor", "patch"],
      "groupName": "Rust minor and patch updates"
    },
    {
      "description": "Group all GitHub Actions updates together",
      "matchManagers": ["github-actions"],
      "matchUpdateTypes": ["minor", "patch"],
      "groupName": "GitHub Actions minor and patch updates"
    },
    {
      "description": "Group all Docker image updates together",
      "matchDatasources": ["docker"],
      "matchUpdateTypes": ["minor", "patch"],
      "groupName": "Docker image minor and patch updates"
    },
  ],
};
