//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsISupportsPriority.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISupportsPriority",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute long priority; */
                    Method {
                        name: "GetPriority",
                        params: &[Param { name: "aPriority", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPriority",
                        params: &[Param { name: "aPriority", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void adjustPriority (in long delta); */
                    Method {
                        name: "AdjustPriority",
                        params: &[Param { name: "delta", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

