//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIProcess.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProcess",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in nsIFile executable); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "executable", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void kill (); */
                    Method {
                        name: "Kill",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count); */
                    Method {
                        name: "Run",
                        params: &[Param { name: "blocking", ty: "bool" }, Param { name: "args", ty: "*mut *const libc::c_char" }, Param { name: "count", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
                    Method {
                        name: "RunAsync",
                        params: &[Param { name: "args", ty: "*mut *const libc::c_char" }, Param { name: "count", ty: "u32" }, Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "holdWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count); */
                    Method {
                        name: "Runw",
                        params: &[Param { name: "blocking", ty: "bool" }, Param { name: "args", ty: "*mut *const i16" }, Param { name: "count", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
                    Method {
                        name: "RunwAsync",
                        params: &[Param { name: "args", ty: "*mut *const i16" }, Param { name: "count", ty: "u32" }, Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "holdWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean startHidden; */
                    Method {
                        name: "GetStartHidden",
                        params: &[Param { name: "aStartHidden", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetStartHidden",
                        params: &[Param { name: "aStartHidden", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean noShell; */
                    Method {
                        name: "GetNoShell",
                        params: &[Param { name: "aNoShell", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetNoShell",
                        params: &[Param { name: "aNoShell", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long pid; */
                    Method {
                        name: "GetPid",
                        params: &[Param { name: "aPid", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long exitValue; */
                    Method {
                        name: "GetExitValue",
                        params: &[Param { name: "aExitValue", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isRunning; */
                    Method {
                        name: "GetIsRunning",
                        params: &[Param { name: "aIsRunning", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

