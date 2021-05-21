//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIDebug2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDebug2",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean isDebugBuild; */
                    Method {
                        name: "GetIsDebugBuild",
                        params: &[Param { name: "aIsDebugBuild", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long assertionCount; */
                    Method {
                        name: "GetAssertionCount",
                        params: &[Param { name: "aAssertionCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isDebuggerAttached; */
                    Method {
                        name: "GetIsDebuggerAttached",
                        params: &[Param { name: "aIsDebuggerAttached", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
                    Method {
                        name: "Assertion",
                        params: &[Param { name: "aStr", ty: "*const libc::c_char" }, Param { name: "aExpr", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void warning (in string aStr, in string aFile, in long aLine); */
                    Method {
                        name: "Warning",
                        params: &[Param { name: "aStr", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void break (in string aFile, in long aLine); */
                    Method {
                        name: "Break",
                        params: &[Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void abort (in string aFile, in long aLine); */
                    Method {
                        name: "Abort",
                        params: &[Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void rustPanic (in string aMessage); */
                    Method {
                        name: "RustPanic",
                        params: &[Param { name: "aMessage", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void rustLog (in string aTarget, in string aMessage); */
                    Method {
                        name: "RustLog",
                        params: &[Param { name: "aTarget", ty: "*const libc::c_char" }, Param { name: "aMessage", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void crashWithOOM (); */
                    Method {
                        name: "CrashWithOOM",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

