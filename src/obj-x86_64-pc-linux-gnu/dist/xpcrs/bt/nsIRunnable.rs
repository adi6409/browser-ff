//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIRunnable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRunnable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void run (); */
                    Method {
                        name: "Run",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIRunnablePriority",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long priority; */
                    Method {
                        name: "GetPriority",
                        params: &[Param { name: "aPriority", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIRunnableIPCMessageType",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

