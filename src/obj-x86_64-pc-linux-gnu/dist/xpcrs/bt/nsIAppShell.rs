//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIAppShell.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAppShell",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void run (); */
                    Method {
                        name: "Run",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void exit (); */
                    Method {
                        name: "Exit",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay); */
                    Method {
                        name: "FavorPerformanceHint",
                        params: &[Param { name: "favorPerfOverStarvation", ty: "bool" }, Param { name: "starvationDelay", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void suspendNative (); */
                    Method {
                        name: "SuspendNative",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void resumeNative (); */
                    Method {
                        name: "ResumeNative",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long eventloopNestingLevel; */
                    Method {
                        name: "GetEventloopNestingLevel",
                        params: &[Param { name: "aEventloopNestingLevel", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

