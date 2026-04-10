import type { HardhatUserConfig } from "hardhat/types/config.js";

const config: HardhatUserConfig = {
  solidity: {
    profiles: {
      default: {
        version: "0.8.24",
      },
    },
  },
};

export default config;