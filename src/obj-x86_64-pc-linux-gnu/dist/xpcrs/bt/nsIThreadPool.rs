//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadPool.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThreadPoolListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onThreadCreated (); */
                    Method {
                        name: "OnThreadCreated",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onThreadShuttingDown (); */
                    Method {
                        name: "OnThreadShuttingDown",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIThreadPool",
            base: Some("nsIEventTarget"),
            methods: Ok(&[
                    /* void shutdown (); */
                    Method {
                        name: "Shutdown",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void shutdownWithTimeout (in long aTimeoutMs); */
                    Method {
                        name: "ShutdownWithTimeout",
                        params: &[Param { name: "aTimeoutMs", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long threadLimit; */
                    Method {
                        name: "GetThreadLimit",
                        params: &[Param { name: "aThreadLimit", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetThreadLimit",
                        params: &[Param { name: "aThreadLimit", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long idleThreadLimit; */
                    Method {
                        name: "GetIdleThreadLimit",
                        params: &[Param { name: "aIdleThreadLimit", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIdleThreadLimit",
                        params: &[Param { name: "aIdleThreadLimit", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long idleThreadTimeout; */
                    Method {
                        name: "GetIdleThreadTimeout",
                        params: &[Param { name: "aIdleThreadTimeout", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIdleThreadTimeout",
                        params: &[Param { name: "aIdleThreadTimeout", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean idleThreadTimeoutRegressive; */
                    Method {
                        name: "GetIdleThreadTimeoutRegressive",
                        params: &[Param { name: "aIdleThreadTimeoutRegressive", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIdleThreadTimeoutRegressive",
                        params: &[Param { name: "aIdleThreadTimeoutRegressive", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long threadStackSize; */
                    Method {
                        name: "GetThreadStackSize",
                        params: &[Param { name: "aThreadStackSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetThreadStackSize",
                        params: &[Param { name: "aThreadStackSize", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIThreadPoolListener listener; */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIThreadPoolListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIThreadPoolListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setName (in ACString aName); */
                    Method {
                        name: "SetName",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

