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

  // Automerge
  platformAutomerge: true,
  automergeType: "pr",

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
      description: "Update Renovate",
      customType: "regex",
      managerFilePatterns: ["/^\\.github/workflows/deps\\.ya?ml$/"],
      matchStrings: [
         "RENOVATE_VERSION:\\s*['\"]?(?<currentValue>[0-9.]+)['\"]?"
      ],
      depNameTemplate: "renovatebot/renovate",
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
      "groupName": "GitHub Actions minor and patch updates",
      "automerge": true
    },
    {
      "description": "Group all Docker image updates together",
      "matchDatasources": ["docker"],
      "matchUpdateTypes": ["minor", "patch"],
      "groupName": "Docker image minor and patch updates"
    },
    {
      "description": "Group all Gradle minor and patch updates together",
      "matchManagers": ["gradle", "gradle-wrapper"],
      "matchUpdateTypes": ["minor", "patch"],
      "groupName": "Gradle minor and patch updates"
    },
    {
      "description": "Pin Cargo dependencies",
      "matchManagers": ["cargo"],
      "rangeStrategy": "pin"
    },
    {
      description: "Update Renovate once a week and automerge",
      matchDepNames: ["renovatebot/renovate"],
      schedule: ["on sunday"],
      automerge: true
    }
  ],
};
