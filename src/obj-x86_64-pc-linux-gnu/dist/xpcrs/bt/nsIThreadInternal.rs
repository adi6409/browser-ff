//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThreadInternal",
            base: Some("nsIThread"),
            methods: Ok(&[
                    /* attribute nsIThreadObserver observer; */
                    Method {
                        name: "GetObserver",
                        params: &[Param { name: "aObserver", ty: "*mut*const nsIThreadObserver" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIThreadObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserver (in nsIThreadObserver observer); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "observer", ty: "*const nsIThreadObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsIThreadObserver observer); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "observer", ty: "*const nsIThreadObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIThreadObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDispatchedEvent (); */
                    Method {
                        name: "OnDispatchedEvent",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait); */
                    Method {
                        name: "OnProcessNextEvent",
                        params: &[Param { name: "thread", ty: "*const nsIThreadInternal" }, Param { name: "mayWait", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed); */
                    Method {
                        name: "AfterProcessNextEvent",
                        params: &[Param { name: "thread", ty: "*const nsIThreadInternal" }, Param { name: "eventWasProcessed", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

