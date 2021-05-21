//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIServiceManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServiceManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "GetService",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "GetServiceByContractID",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID); */
                    Method {
                        name: "IsServiceInstantiated",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID); */
                    Method {
                        name: "IsServiceInstantiatedByContractID",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

