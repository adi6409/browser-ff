//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIComponentManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIComponentManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "GetClassObject",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "GetClassObjectByContractID",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "CreateInstance",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aDelegate", ty: "*const nsISupports" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "CreateInstanceByContractID",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aDelegate", ty: "*const nsISupports" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addBootstrappedManifestLocation (in nsIFile aLocation); */
                    Method {
                        name: "AddBootstrappedManifestLocation",
                        params: &[Param { name: "aLocation", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeBootstrappedManifestLocation (in nsIFile aLocation); */
                    Method {
                        name: "RemoveBootstrappedManifestLocation",
                        params: &[Param { name: "aLocation", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIArray getManifestLocations (); */
                    Method {
                        name: "GetManifestLocations",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUTF8StringEnumerator getComponentJSMs (); */
                    Method {
                        name: "GetComponentJSMs",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

