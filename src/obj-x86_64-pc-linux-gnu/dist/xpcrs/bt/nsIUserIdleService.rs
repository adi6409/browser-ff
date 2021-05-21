//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIUserIdleService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUserIdleService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long idleTime; */
                    Method {
                        name: "GetIdleTime",
                        params: &[Param { name: "aIdleTime", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
                    Method {
                        name: "AddIdleObserver",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "time", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
                    Method {
                        name: "RemoveIdleObserver",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "time", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean disabled; */
                    Method {
                        name: "GetDisabled",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDisabled",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

