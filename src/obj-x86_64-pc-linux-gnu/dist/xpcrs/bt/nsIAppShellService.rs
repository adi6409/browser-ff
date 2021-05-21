//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIAppShellService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAppShellService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIAppWindow createTopLevelWindow (in nsIAppWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight); */
                    Method {
                        name: "CreateTopLevelWindow",
                        params: &[Param { name: "aParent", ty: "*const nsIAppWindow" }, Param { name: "aUrl", ty: "*const nsIURI" }, Param { name: "aChromeMask", ty: "uint32_t" }, Param { name: "aInitialWidth", ty: "i32" }, Param { name: "aInitialHeight", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome, [optional] in uint32_t aChromeMask); */
                    Method {
                        name: "CreateWindowlessBrowser",
                        params: &[Param { name: "aIsChrome", ty: "bool" }, Param { name: "aChromeMask", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut*const nsIWindowlessBrowser" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void createHiddenWindow (); */
                    Method {
                        name: "CreateHiddenWindow",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void destroyHiddenWindow (); */
                    Method {
                        name: "DestroyHiddenWindow",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void setScreenId (in uint32_t aScreenId); */
                    Method {
                        name: "SetScreenId",
                        params: &[Param { name: "aScreenId", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAppWindow hiddenWindow; */
                    Method {
                        name: "GetHiddenWindow",
                        params: &[Param { name: "aHiddenWindow", ty: "*mut*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
                    Method {
                        name: "GetHiddenDOMWindow",
                        params: &[Param { name: "aHiddenDOMWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean applicationProvidedHiddenWindow; */
                    Method {
                        name: "GetApplicationProvidedHiddenWindow",
                        params: &[Param { name: "aApplicationProvidedHiddenWindow", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerTopLevelWindow (in nsIAppWindow aWindow); */
                    Method {
                        name: "RegisterTopLevelWindow",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterTopLevelWindow (in nsIAppWindow aWindow); */
                    Method {
                        name: "UnregisterTopLevelWindow",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasHiddenWindow; */
                    Method {
                        name: "GetHasHiddenWindow",
                        params: &[Param { name: "aHasHiddenWindow", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool startEventLoopLagTracking (); */
                    Method {
                        name: "StartEventLoopLagTracking",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopEventLoopLagTracking (); */
                    Method {
                        name: "StopEventLoopLagTracking",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

