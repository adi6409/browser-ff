//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowMediator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
                    Method {
                        name: "GetEnumerator",
                        params: &[Param { name: "aWindowType", ty: "*const i16" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator getAppWindowEnumerator (in wstring aWindowType); */
                    Method {
                        name: "GetAppWindowEnumerator",
                        params: &[Param { name: "aWindowType", ty: "*const i16" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator getZOrderAppWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
                    Method {
                        name: "GetZOrderAppWindowEnumerator",
                        params: &[Param { name: "aWindowType", ty: "*const i16" }, Param { name: "aFrontToBack", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
                    Method {
                        name: "GetMostRecentWindow",
                        params: &[Param { name: "aWindowType", ty: "*const i16" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindowProxy getMostRecentBrowserWindow (); */
                    Method {
                        name: "GetMostRecentBrowserWindow",
                        params: &[Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
                    Method {
                        name: "GetMostRecentNonPBWindow",
                        params: &[Param { name: "aWindowType", ty: "*const i16" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
                    Method {
                        name: "GetOuterWindowWithId",
                        params: &[Param { name: "aOuterWindowID", ty: "u64" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
                    Method {
                        name: "GetCurrentInnerWindowWithId",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void registerWindow (in nsIAppWindow aWindow); */
                    Method {
                        name: "RegisterWindow",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void unregisterWindow (in nsIAppWindow aWindow); */
                    Method {
                        name: "UnregisterWindow",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void updateWindowTimeStamp (in nsIAppWindow aWindow); */
                    Method {
                        name: "UpdateWindowTimeStamp",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] boolean calculateZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
                    Method {
                        name: "CalculateZPosition",
                        params: &[Param { name: "inWindow", ty: "*const nsIAppWindow" }, Param { name: "inPosition", ty: "u32" }, Param { name: "inBelow", ty: "*const nsIWidget" }, Param { name: "outPosition", ty: "*mut u32" }, Param { name: "outBelow", ty: "*mut*const nsIWidget" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void setZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIAppWindow inBelow); */
                    Method {
                        name: "SetZPosition",
                        params: &[Param { name: "inWindow", ty: "*const nsIAppWindow" }, Param { name: "inPosition", ty: "u32" }, Param { name: "inBelow", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] uint32_t getZLevel (in nsIAppWindow aWindow); */
                    Method {
                        name: "GetZLevel",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void setZLevel (in nsIAppWindow aWindow, in uint32_t aZLevel); */
                    Method {
                        name: "SetZLevel",
                        params: &[Param { name: "aWindow", ty: "*const nsIAppWindow" }, Param { name: "aZLevel", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addListener (in nsIWindowMediatorListener aListener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "aListener", ty: "*const nsIWindowMediatorListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeListener (in nsIWindowMediatorListener aListener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "aListener", ty: "*const nsIWindowMediatorListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

