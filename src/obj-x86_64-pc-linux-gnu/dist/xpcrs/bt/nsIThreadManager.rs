//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINestedEventLoopCondition",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool isDone (); */
                    Method {
                        name: "IsDone",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIThreadManager",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        ]; D}

