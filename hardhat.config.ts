import type { HardhatUserConfig } from "hardhat/types/config.js";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.24",
  },
  networks: {
    hardhat: {
      type: "edr-simulated",
    },
  },
};

export default config;