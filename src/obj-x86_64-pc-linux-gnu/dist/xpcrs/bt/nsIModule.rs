//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIModule",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
                    Method {
                        name: "GetClassObject",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "aResult", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
                    Method {
                        name: "RegisterSelf",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aLocation", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }, Param { name: "aType", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
                    Method {
                        name: "UnregisterSelf",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aLocation", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canUnload (in nsIComponentManager aCompMgr); */
                    Method {
                        name: "CanUnload",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

