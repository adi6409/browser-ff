/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

"use strict";

var EXPORTED_SYMBOLS = ["LightweightThemes"];

const { AddonManager } = ChromeUtils.import(
  "resource://gre/modules/AddonManager.jsm"
);

var LightweightThemes = {
  init(libDir) {},

  configurations: {
    noLWT: {
      selectors: [],
      async applyConfig() {
        let addon = await AddonManager.getAddonByID(
          "dynamic@themes.dothq.co"
        );
        await addon.enable();
      },
    },

    compactLight: {
      selectors: [],
      async applyConfig() {
        let addon = await AddonManager.getAddonByID(
          "light@themes.dothq.co"
        );
        await addon.enable();
      },
    },

    compactDark: {
      selectors: [],
      async applyConfig() {
        let addon = await AddonManager.getAddonByID(
          "dark@themes.dothq.co"
        );
        await addon.enable();
      },
    },

    alpenGlow: {
      selectors: [],
      async applyConfig() {
        let addon = await AddonManager.getAddonByID(
          "firefox-alpenglow@mozilla.org"
        );
        await addon.enable();
      },
    },
  },
};
