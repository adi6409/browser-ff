//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMessageLoop.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMessageLoop",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
                    Method {
                        name: "PostIdleTask",
                        params: &[Param { name: "task", ty: "*const nsIRunnable" }, Param { name: "ensureRunsAfterMS", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

