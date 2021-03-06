/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

"use strict";

const { generateActorSpec, Arg, types } = require("devtools/shared/protocol");

types.addDictType("breakpoint-list.breakpoint-options", {
  condition: "nullable:string",
  logValue: "nullable:string",
});

const breakpointListSpec = generateActorSpec({
  typeName: "breakpoint-list",

  methods: {
    setBreakpoint: {
      request: {
        location: Arg(0, "json"),
        options: Arg(1, "breakpoint-list.breakpoint-options"),
      },
    },
    removeBreakpoint: {
      request: {
        location: Arg(0, "json"),
      },
    },
  },
});

exports.breakpointListSpec = breakpointListSpec;
