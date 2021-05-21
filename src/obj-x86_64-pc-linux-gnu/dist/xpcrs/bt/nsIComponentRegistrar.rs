//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIComponentRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIComponentRegistrar",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void autoRegister (in nsIFile aSpec); */
                    Method {
                        name: "AutoRegister",
                        params: &[Param { name: "aSpec", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void autoUnregister (in nsIFile aSpec); */
                    Method {
                        name: "AutoUnregister",
                        params: &[Param { name: "aSpec", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory); */
                    Method {
                        name: "RegisterFactory",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aClassName", ty: "*const libc::c_char" }, Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aFactory", ty: "*const nsIFactory" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory); */
                    Method {
                        name: "UnregisterFactory",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aFactory", ty: "*const nsIFactory" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType); */
                    Method {
                        name: "RegisterFactoryLocation",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aClassName", ty: "*const libc::c_char" }, Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }, Param { name: "aType", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile); */
                    Method {
                        name: "UnregisterFactoryLocation",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCIDRegistered (in nsCIDRef aClass); */
                    Method {
                        name: "IsCIDRegistered",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isContractIDRegistered (in string aContractID); */
                    Method {
                        name: "IsContractIDRegistered",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getContractIDs (); */
                    Method {
                        name: "GetContractIDs",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string CIDToContractID (in nsCIDRef aClass); */
                    Method {
                        name: "CIDToContractID",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsCIDPtr contractIDToCID (in string aContractID); */
                    Method {
                        name: "ContractIDToCID",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *mut nsCID" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

